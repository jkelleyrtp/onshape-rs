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
pub struct BtCompanyActivateParams {
    #[serde(rename = "planId", skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "seats", skip_serializing_if = "Option::is_none")]
    pub seats: Option<i64>,
    #[serde(rename = "paymentType", skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<i32>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl BtCompanyActivateParams {
    pub fn new() -> BtCompanyActivateParams {
        BtCompanyActivateParams {
            plan_id: None,
            seats: None,
            payment_type: None,
            user_id: None,
        }
    }
}


