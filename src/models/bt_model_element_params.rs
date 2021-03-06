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
pub struct BtModelElementParams {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl BtModelElementParams {
    pub fn new() -> BtModelElementParams {
        BtModelElementParams {
            name: None,
        }
    }
}


