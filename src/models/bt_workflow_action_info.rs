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
pub struct BtWorkflowActionInfo {
    #[serde(rename = "requiredProperties", skip_serializing_if = "Option::is_none")]
    pub required_properties: Option<Vec<String>>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "uiHint", skip_serializing_if = "Option::is_none")]
    pub ui_hint: Option<String>,
    #[serde(rename = "isApproverAction", skip_serializing_if = "Option::is_none")]
    pub is_approver_action: Option<bool>,
    #[serde(rename = "isAdminOverride", skip_serializing_if = "Option::is_none")]
    pub is_admin_override: Option<bool>,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "allowIfNoApprovers", skip_serializing_if = "Option::is_none")]
    pub allow_if_no_approvers: Option<bool>,
    #[serde(rename = "allowIfApprovers", skip_serializing_if = "Option::is_none")]
    pub allow_if_approvers: Option<bool>,
    #[serde(rename = "alwaysAllow", skip_serializing_if = "Option::is_none")]
    pub always_allow: Option<bool>,
    #[serde(rename = "tooltip", skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<String>,
}

impl BtWorkflowActionInfo {
    pub fn new() -> BtWorkflowActionInfo {
        BtWorkflowActionInfo {
            required_properties: None,
            action: None,
            ui_hint: None,
            is_approver_action: None,
            is_admin_override: None,
            label: None,
            allow_if_no_approvers: None,
            allow_if_approvers: None,
            always_allow: None,
            tooltip: None,
        }
    }
}

