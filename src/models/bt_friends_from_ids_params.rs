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
pub struct BtFriendsFromIdsParams {
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

impl BtFriendsFromIdsParams {
    pub fn new() -> BtFriendsFromIdsParams {
        BtFriendsFromIdsParams {
            ids: None,
        }
    }
}

