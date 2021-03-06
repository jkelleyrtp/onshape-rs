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
pub struct BtAdminRolePermissionParams {
    #[serde(rename = "resourcePath", skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
    #[serde(rename = "resourceLabel", skip_serializing_if = "Option::is_none")]
    pub resource_label: Option<String>,
    #[serde(rename = "allowed", skip_serializing_if = "Option::is_none")]
    pub allowed: Option<bool>,
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

impl BtAdminRolePermissionParams {
    pub fn new() -> BtAdminRolePermissionParams {
        BtAdminRolePermissionParams {
            resource_path: None,
            resource_label: None,
            allowed: None,
            method: None,
        }
    }
}


