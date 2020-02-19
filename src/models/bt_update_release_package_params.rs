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
pub struct BtUpdateReleasePackageParams {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::BtReleasePackageItemParams>>,
    #[serde(rename = "itemIds", skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
    #[serde(rename = "empty", skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<crate::models::BtPropertyValueParam>>,
}

impl BtUpdateReleasePackageParams {
    pub fn new() -> BtUpdateReleasePackageParams {
        BtUpdateReleasePackageParams {
            items: None,
            item_ids: None,
            empty: None,
            properties: None,
        }
    }
}

