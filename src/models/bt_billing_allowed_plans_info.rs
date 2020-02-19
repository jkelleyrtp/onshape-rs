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
pub struct BtBillingAllowedPlansInfo {
    #[serde(rename = "currentPlan", skip_serializing_if = "Option::is_none")]
    pub current_plan: Option<crate::models::BtBillingPlanSummaryInfo>,
    #[serde(rename = "upgrades", skip_serializing_if = "Option::is_none")]
    pub upgrades: Option<Vec<crate::models::BtBillingPlanSummaryInfo>>,
    #[serde(rename = "downgrades", skip_serializing_if = "Option::is_none")]
    pub downgrades: Option<Vec<crate::models::BtBillingPlanSummaryInfo>>,
}

impl BtBillingAllowedPlansInfo {
    pub fn new() -> BtBillingAllowedPlansInfo {
        BtBillingAllowedPlansInfo {
            current_plan: None,
            upgrades: None,
            downgrades: None,
        }
    }
}

