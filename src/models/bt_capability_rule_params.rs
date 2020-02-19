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
pub struct BtCapabilityRuleParams {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "script", skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
}

impl BtCapabilityRuleParams {
    pub fn new() -> BtCapabilityRuleParams {
        BtCapabilityRuleParams {
            name: None,
            id: None,
            description: None,
            script: None,
        }
    }
}


