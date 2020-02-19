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
pub struct BtColorParams {
    #[serde(rename = "red", skip_serializing_if = "Option::is_none")]
    pub red: Option<i32>,
    #[serde(rename = "green", skip_serializing_if = "Option::is_none")]
    pub green: Option<i32>,
    #[serde(rename = "blue", skip_serializing_if = "Option::is_none")]
    pub blue: Option<i32>,
}

impl BtColorParams {
    pub fn new() -> BtColorParams {
        BtColorParams {
            red: None,
            green: None,
            blue: None,
        }
    }
}


