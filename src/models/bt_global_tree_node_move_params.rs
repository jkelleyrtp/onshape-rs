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
pub struct BtGlobalTreeNodeMoveParams {
    #[serde(rename = "itemsToMove", skip_serializing_if = "Option::is_none")]
    pub items_to_move: Option<Vec<crate::models::Item>>,
}

impl BtGlobalTreeNodeMoveParams {
    pub fn new() -> BtGlobalTreeNodeMoveParams {
        BtGlobalTreeNodeMoveParams {
            items_to_move: None,
        }
    }
}


