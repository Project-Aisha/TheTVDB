/*
 * TVDB API V4
 *
 * Documentation of [TheTVDB](https://thetvdb.com/) API V4. All related information is linked from our [Github repo](https://github.com/thetvdb/v4-api). You might also want to use our [Postman collection] (https://www.getpostman.com/collections/7a9397ce69ff246f74d0) ## Authentication 1. Use the /login endpoint and provide your API key as \"apikey\". If you have a user-supported key, also provide your subscriber PIN as \"pin\". Otherwise completely remove \"pin\" from your call. 2. Executing this call will provide you with a bearer token, which is valid for 1 month. 3. Provide your bearer token for subsequent API calls by clicking Authorize below or including in the header of all direct API calls: `Authorization: Bearer [your-token]`  ## Notes 1. \"score\" is a field across almost all entities.  We generate scores for different types of entities in various ways, so no assumptions should be made about the meaning of this value.  It is simply used to hint at relative popularity for sorting purposes.
 *
 * The version of the OpenAPI document: 4.7.10
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SeriesBaseRecord : The base record for a series. All series airs time like firstAired, lastAired, nextAired, etc. are in US EST for US series, and for all non-US series, the time of the show’s country capital or most populous city. For streaming services, is the official release time. See https://support.thetvdb.com/kb/faq.php?id=29.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeriesBaseRecord {
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<models::Alias>>,
    #[serde(
        rename = "averageRuntime",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub average_runtime: Option<Option<i32>>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "defaultSeasonType", skip_serializing_if = "Option::is_none")]
    pub default_season_type: Option<i64>,
    #[serde(rename = "episodes", skip_serializing_if = "Option::is_none")]
    pub episodes: Option<Vec<models::EpisodeBaseRecord>>,
    #[serde(rename = "firstAired", skip_serializing_if = "Option::is_none")]
    pub first_aired: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "isOrderRandomized", skip_serializing_if = "Option::is_none")]
    pub is_order_randomized: Option<bool>,
    #[serde(rename = "lastAired", skip_serializing_if = "Option::is_none")]
    pub last_aired: Option<String>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nameTranslations", skip_serializing_if = "Option::is_none")]
    pub name_translations: Option<Vec<String>>,
    #[serde(rename = "nextAired", skip_serializing_if = "Option::is_none")]
    pub next_aired: Option<String>,
    #[serde(rename = "originalCountry", skip_serializing_if = "Option::is_none")]
    pub original_country: Option<String>,
    #[serde(rename = "originalLanguage", skip_serializing_if = "Option::is_none")]
    pub original_language: Option<String>,
    #[serde(
        rename = "overviewTranslations",
        skip_serializing_if = "Option::is_none"
    )]
    pub overview_translations: Option<Vec<String>>,
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<models::Status>>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<String>,
}

impl SeriesBaseRecord {
    /// The base record for a series. All series airs time like firstAired, lastAired, nextAired, etc. are in US EST for US series, and for all non-US series, the time of the show’s country capital or most populous city. For streaming services, is the official release time. See https://support.thetvdb.com/kb/faq.php?id=29.
    pub fn new() -> SeriesBaseRecord {
        SeriesBaseRecord {
            aliases: None,
            average_runtime: None,
            country: None,
            default_season_type: None,
            episodes: None,
            first_aired: None,
            id: None,
            image: None,
            is_order_randomized: None,
            last_aired: None,
            last_updated: None,
            name: None,
            name_translations: None,
            next_aired: None,
            original_country: None,
            original_language: None,
            overview_translations: None,
            score: None,
            slug: None,
            status: None,
            year: None,
        }
    }
}
