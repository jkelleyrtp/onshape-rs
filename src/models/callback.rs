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
pub struct Callback {
    #[serde(rename = "extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "get$ref", skip_serializing_if = "Option::is_none")]
    pub getref: Option<String>,
    #[serde(rename = "empty", skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
}

impl Callback {
    pub fn new() -> Callback {
        Callback {
            extensions: None,
            getref: None,
            empty: None,
        }
    }
}


