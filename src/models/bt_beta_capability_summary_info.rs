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
pub struct BtBetaCapabilitySummaryInfo {
    #[serde(rename = "userState", skip_serializing_if = "Option::is_none")]
    pub user_state: Option<i32>,
    #[serde(rename = "upgradable", skip_serializing_if = "Option::is_none")]
    pub upgradable: Option<bool>,
    #[serde(rename = "userBased", skip_serializing_if = "Option::is_none")]
    pub user_based: Option<bool>,
    #[serde(rename = "selfServiceDisabled", skip_serializing_if = "Option::is_none")]
    pub self_service_disabled: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl BtBetaCapabilitySummaryInfo {
    pub fn new() -> BtBetaCapabilitySummaryInfo {
        BtBetaCapabilitySummaryInfo {
            user_state: None,
            upgradable: None,
            user_based: None,
            self_service_disabled: None,
            description: None,
            href: None,
            view_ref: None,
            name: None,
            id: None,
        }
    }
}


