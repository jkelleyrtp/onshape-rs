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
pub struct BtRbacPermissionSchemeListInfo {
    #[serde(rename = "defaultIndex", skip_serializing_if = "Option::is_none")]
    pub default_index: Option<i32>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::BtRbacPermissionSchemeInfo>>,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

impl BtRbacPermissionSchemeListInfo {
    pub fn new() -> BtRbacPermissionSchemeListInfo {
        BtRbacPermissionSchemeListInfo {
            default_index: None,
            href: None,
            items: None,
            previous: None,
            next: None,
        }
    }
}

