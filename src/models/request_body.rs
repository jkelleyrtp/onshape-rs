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
pub struct RequestBody {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<::std::collections::HashMap<String, crate::models::MediaType>>,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "get$ref", skip_serializing_if = "Option::is_none")]
    pub getref: Option<String>,
}

impl RequestBody {
    pub fn new() -> RequestBody {
        RequestBody {
            description: None,
            content: None,
            required: None,
            extensions: None,
            getref: None,
        }
    }
}

