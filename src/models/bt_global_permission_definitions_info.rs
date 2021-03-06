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
pub struct BtGlobalPermissionDefinitionsInfo {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::Definition>>,
}

impl BtGlobalPermissionDefinitionsInfo {
    pub fn new() -> BtGlobalPermissionDefinitionsInfo {
        BtGlobalPermissionDefinitionsInfo {
            items: None,
        }
    }
}


