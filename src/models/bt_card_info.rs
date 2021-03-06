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
pub struct BtCardInfo {
    #[serde(rename = "last4", skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[serde(rename = "expMonth", skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i32>,
    #[serde(rename = "expYear", skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i32>,
    #[serde(rename = "billingAddress", skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<crate::models::BtAddressInfo>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl BtCardInfo {
    pub fn new() -> BtCardInfo {
        BtCardInfo {
            last4: None,
            exp_month: None,
            exp_year: None,
            billing_address: None,
            _type: None,
        }
    }
}


