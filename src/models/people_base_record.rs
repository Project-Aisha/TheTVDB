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

/// PeopleBaseRecord : base people record
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeopleBaseRecord {
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<models::Alias>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nameTranslations", skip_serializing_if = "Option::is_none")]
    pub name_translations: Option<Vec<String>>,
    #[serde(
        rename = "overviewTranslations",
        skip_serializing_if = "Option::is_none"
    )]
    pub overview_translations: Option<Vec<String>>,
    #[serde(rename = "score", skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
}

impl PeopleBaseRecord {
    /// base people record
    pub fn new() -> PeopleBaseRecord {
        PeopleBaseRecord {
            aliases: None,
            id: None,
            image: None,
            last_updated: None,
            name: None,
            name_translations: None,
            overview_translations: None,
            score: None,
        }
    }
}
