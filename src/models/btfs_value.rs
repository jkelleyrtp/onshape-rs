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
pub struct BtfsValue {
    #[serde(rename = "valueObject", skip_serializing_if = "Option::is_none")]
    pub value_object: Option<serde_json::Value>,
    #[serde(rename = "configurationValueString", skip_serializing_if = "Option::is_none")]
    pub configuration_value_string: Option<String>,
    #[serde(rename = "typeTag", skip_serializing_if = "Option::is_none")]
    pub type_tag: Option<String>,
}

impl BtfsValue {
    pub fn new() -> BtfsValue {
        BtfsValue {
            value_object: None,
            configuration_value_string: None,
            type_tag: None,
        }
    }
}


