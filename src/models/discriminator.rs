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
pub struct Discriminator {
    #[serde(rename = "propertyName", skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,
    #[serde(rename = "mapping", skip_serializing_if = "Option::is_none")]
    pub mapping: Option<::std::collections::HashMap<String, String>>,
}

impl Discriminator {
    pub fn new() -> Discriminator {
        Discriminator {
            property_name: None,
            mapping: None,
        }
    }
}


