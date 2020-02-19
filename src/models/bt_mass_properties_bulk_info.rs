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
pub struct BtMassPropertiesBulkInfo {
    #[serde(rename = "microversionId", skip_serializing_if = "Option::is_none")]
    pub microversion_id: Option<String>,
    #[serde(rename = "bodies", skip_serializing_if = "Option::is_none")]
    pub bodies: Option<::std::collections::HashMap<String, crate::models::BtMassPropertiesInfo>>,
}

impl BtMassPropertiesBulkInfo {
    pub fn new() -> BtMassPropertiesBulkInfo {
        BtMassPropertiesBulkInfo {
            microversion_id: None,
            bodies: None,
        }
    }
}


