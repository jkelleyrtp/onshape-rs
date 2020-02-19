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
pub struct Entry {
    #[serde(rename = "permissionSet", skip_serializing_if = "Option::is_none")]
    pub permission_set: Option<serde_json::Value>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::models::BtRbacRoleInfo>,
}

impl Entry {
    pub fn new() -> Entry {
        Entry {
            permission_set: None,
            role: None,
        }
    }
}


