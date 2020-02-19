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
pub struct ExternalAccount {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "instanceURL", skip_serializing_if = "Option::is_none")]
    pub instance_url: Option<String>,
}

impl ExternalAccount {
    pub fn new() -> ExternalAccount {
        ExternalAccount {
            id: None,
            object: None,
            account: None,
            customer: None,
            metadata: None,
            instance_url: None,
        }
    }
}


