/*
 * Onshape REST API
 *
 * The Onshape REST API consumed by all clients.
 *
 * The version of the OpenAPI document: 1.104
 * Contact: api-support@onshape.zendesk.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BtCacheItemInfo {
    #[serde(rename = "cacheName", skip_serializing_if = "Option::is_none")]
    pub cache_name: Option<String>,
    #[serde(rename = "cacheKey", skip_serializing_if = "Option::is_none")]
    pub cache_key: Option<String>,
    #[serde(rename = "lastModifiedAt", skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<i64>,
    #[serde(rename = "cached", skip_serializing_if = "Option::is_none")]
    pub cached: Option<bool>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl BtCacheItemInfo {
    pub fn new() -> BtCacheItemInfo {
        BtCacheItemInfo {
            cache_name: None,
            cache_key: None,
            last_modified_at: None,
            cached: None,
            href: None,
            view_ref: None,
            name: None,
            id: None,
        }
    }
}


