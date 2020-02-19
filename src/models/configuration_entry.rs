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
pub struct ConfigurationEntry {
    #[serde(rename = "parameterId", skip_serializing_if = "Option::is_none")]
    pub parameter_id: Option<String>,
    #[serde(rename = "parameterValue", skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
}

impl ConfigurationEntry {
    pub fn new() -> ConfigurationEntry {
        ConfigurationEntry {
            parameter_id: None,
            parameter_value: None,
        }
    }
}


