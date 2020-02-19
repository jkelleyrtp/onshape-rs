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
pub struct PathItem {
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "get", skip_serializing_if = "Option::is_none")]
    pub get: Option<crate::models::Operation>,
    #[serde(rename = "put", skip_serializing_if = "Option::is_none")]
    pub put: Option<crate::models::Operation>,
    #[serde(rename = "post", skip_serializing_if = "Option::is_none")]
    pub post: Option<crate::models::Operation>,
    #[serde(rename = "delete", skip_serializing_if = "Option::is_none")]
    pub delete: Option<crate::models::Operation>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<crate::models::Operation>,
    #[serde(rename = "head", skip_serializing_if = "Option::is_none")]
    pub head: Option<crate::models::Operation>,
    #[serde(rename = "patch", skip_serializing_if = "Option::is_none")]
    pub patch: Option<crate::models::Operation>,
    #[serde(rename = "trace", skip_serializing_if = "Option::is_none")]
    pub trace: Option<crate::models::Operation>,
    #[serde(rename = "servers", skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<crate::models::Server>>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::Parameter>>,
    #[serde(rename = "get$ref", skip_serializing_if = "Option::is_none")]
    pub getref: Option<String>,
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl PathItem {
    pub fn new() -> PathItem {
        PathItem {
            summary: None,
            description: None,
            get: None,
            put: None,
            post: None,
            delete: None,
            options: None,
            head: None,
            patch: None,
            trace: None,
            servers: None,
            parameters: None,
            getref: None,
            extensions: None,
        }
    }
}

