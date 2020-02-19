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
pub struct BtRbacPermissionSchemeInfo {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<crate::models::Entry>>,
    #[serde(rename = "predefinedPermissionScheme", skip_serializing_if = "Option::is_none")]
    pub predefined_permission_scheme: Option<i32>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl BtRbacPermissionSchemeInfo {
    pub fn new() -> BtRbacPermissionSchemeInfo {
        BtRbacPermissionSchemeInfo {
            description: None,
            entries: None,
            predefined_permission_scheme: None,
            active: None,
            href: None,
            view_ref: None,
            name: None,
            id: None,
        }
    }
}


