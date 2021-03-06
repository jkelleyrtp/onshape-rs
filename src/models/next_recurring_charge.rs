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
pub struct NextRecurringCharge {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl NextRecurringCharge {
    pub fn new() -> NextRecurringCharge {
        NextRecurringCharge {
            amount: None,
            date: None,
        }
    }
}


