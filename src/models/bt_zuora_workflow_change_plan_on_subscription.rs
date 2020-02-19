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
pub struct BtZuoraWorkflowChangePlanOnSubscription {
    #[serde(rename = "oldSubscriptionId", skip_serializing_if = "Option::is_none")]
    pub old_subscription_id: Option<String>,
    #[serde(rename = "oldBTBillingPlanId", skip_serializing_if = "Option::is_none")]
    pub old_bt_billing_plan_id: Option<String>,
    #[serde(rename = "newBTBillingPlanId", skip_serializing_if = "Option::is_none")]
    pub new_bt_billing_plan_id: Option<String>,
    #[serde(rename = "effectiveDate", skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(rename = "newSubscriptionId", skip_serializing_if = "Option::is_none")]
    pub new_subscription_id: Option<String>,
}

impl BtZuoraWorkflowChangePlanOnSubscription {
    pub fn new() -> BtZuoraWorkflowChangePlanOnSubscription {
        BtZuoraWorkflowChangePlanOnSubscription {
            old_subscription_id: None,
            old_bt_billing_plan_id: None,
            new_bt_billing_plan_id: None,
            effective_date: None,
            new_subscription_id: None,
        }
    }
}

