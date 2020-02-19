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
pub struct BtIdentity {
    #[serde(rename = "identityType", skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<IdentityType>,
    #[serde(rename = "identityId", skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
}

impl BtIdentity {
    pub fn new() -> BtIdentity {
        BtIdentity {
            identity_type: None,
            identity_id: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IdentityType {
    #[serde(rename = "USER")]
    USER,
    #[serde(rename = "TEAM")]
    TEAM,
}

