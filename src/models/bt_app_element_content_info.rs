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
pub struct BtAppElementContentInfo {
    #[serde(rename = "changeId", skip_serializing_if = "Option::is_none")]
    pub change_id: Option<String>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::BtAppElementContentEntryInfo>>,
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    #[serde(rename = "errorValue", skip_serializing_if = "Option::is_none")]
    pub error_value: Option<ErrorValue>,
    #[serde(rename = "errorDescription", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}

impl BtAppElementContentInfo {
    pub fn new() -> BtAppElementContentInfo {
        BtAppElementContentInfo {
            change_id: None,
            data: None,
            error_code: None,
            error_value: None,
            error_description: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ErrorValue {
    #[serde(rename = "OK")]
    OK,
    #[serde(rename = "TRANSACTION_CONFLICT")]
    TRANSACTIONCONFLICT,
    #[serde(rename = "NOT_FOUND")]
    NOTFOUND,
    #[serde(rename = "INCONSISTENT_CHANGES")]
    INCONSISTENTCHANGES,
}

