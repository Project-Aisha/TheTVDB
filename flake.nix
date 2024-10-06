{
  description = "TheTVDB";

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  inputs = {
    devenv.url = "github:cachix/devenv";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/x86_64-linux";

    devenv-root = {
      url = "file+file:///dev/null";
      flake = false;
    };
  };

  outputs =
    inputs@{ flake-parts, devenv-root, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
      ];

      systems = import inputs.systems;

      perSystem =
        { pkgs, ... }:
        {
          devenv.shells = rec {
            default = {
              devenv.root =
                let
                  devenvRootFileContent = builtins.readFile devenv-root.outPath;
                in
                pkgs.lib.mkIf (devenvRootFileContent != "") devenvRootFileContent;

              name = "TheTVDB";

              packages = with pkgs; [
                openapi-generator-cli
                openssl
              ];

              languages = {
                nix.enable = true;
                rust.enable = true;
              };

              scripts.generate-thetvdb-client.exec = ''
                curl -sSL -o /tmp/thetvdb-swagger.yml https://thetvdb.github.io/v4-api/swagger.yml

                openapi-generator-cli generate \
                  --input-spec "/tmp/thetvdb-swagger.yml" \
                  --generator-name "rust" \
                  --output "." \
                  --package-name "thetvdb" \
                  --additional-properties "supportMiddleware=true" \
                  --minimal-update

                find . -type f -name "*.rs" -exec sed -i 's/season-type/season_type/g' {} +

                cargo fetch
                cargo fmt --all
              '';
            };
            development = pkgs.lib.mkMerge [
              {
                name = "TheTVDB (Development)";

                pre-commit.hooks = {
                  clippy.enable = true;
                  deadnix.enable = true;
                  rustfmt.enable = true;
                  statix.enable = true;
                  nixfmt-rfc-style.enable = true;
                };
              }
              default
            ];
            github-actions = pkgs.lib.mkMerge [
              {
                name = "TheTVDB (GitHub Actions)";

                languages = {
                  rust = {
                    components = [
                      "rustc"
                      "cargo"
                      "rustfmt"
                    ];
                  };
                };
              }
              default
            ];
          };

          formatter = pkgs.nixfmt-rfc-style;
        };
    };
}
