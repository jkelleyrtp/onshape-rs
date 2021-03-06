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
pub struct BtListResponseBtStandardContentHierarchyInfo {
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::BtStandardContentHierarchyInfo>>,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

impl BtListResponseBtStandardContentHierarchyInfo {
    pub fn new() -> BtListResponseBtStandardContentHierarchyInfo {
        BtListResponseBtStandardContentHierarchyInfo {
            href: None,
            items: None,
            previous: None,
            next: None,
        }
    }
}


