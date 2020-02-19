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
pub struct BtUpdateWorkflowParams {
    #[serde(rename = "isPickable", skip_serializing_if = "Option::is_none")]
    pub is_pickable: Option<bool>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl BtUpdateWorkflowParams {
    pub fn new() -> BtUpdateWorkflowParams {
        BtUpdateWorkflowParams {
            is_pickable: None,
            id: None,
        }
    }
}

