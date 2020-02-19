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
pub struct BtTeamInfoAllOf {
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(rename = "member", skip_serializing_if = "Option::is_none")]
    pub member: Option<bool>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl BtTeamInfoAllOf {
    pub fn new() -> BtTeamInfoAllOf {
        BtTeamInfoAllOf {
            admin: None,
            member: None,
            size: None,
        }
    }
}

