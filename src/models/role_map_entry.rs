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
pub struct RoleMapEntry {
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<crate::models::BtRbacRoleInfo>,
    #[serde(rename = "identities", skip_serializing_if = "Option::is_none")]
    pub identities: Option<Vec<crate::models::BtIdentityInfo>>,
}

impl RoleMapEntry {
    pub fn new() -> RoleMapEntry {
        RoleMapEntry {
            role: None,
            identities: None,
        }
    }
}


