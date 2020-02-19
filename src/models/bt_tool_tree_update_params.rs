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
pub struct BtToolTreeUpdateParams {
    #[serde(rename = "remove", skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(rename = "save", skip_serializing_if = "Option::is_none")]
    pub save: Option<Vec<crate::models::BtToolTreeParams>>,
}

impl BtToolTreeUpdateParams {
    pub fn new() -> BtToolTreeUpdateParams {
        BtToolTreeUpdateParams {
            remove: None,
            save: None,
        }
    }
}

