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
pub struct NextCharge {
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(rename = "currentPeriodEnd", skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<String>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl NextCharge {
    pub fn new() -> NextCharge {
        NextCharge {
            interval: None,
            amount: None,
            current_period_end: None,
            total: None,
        }
    }
}


