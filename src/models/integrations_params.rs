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
pub struct IntegrationsParams {
    #[serde(rename = "teamIds", skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,
    #[serde(rename = "providersType", skip_serializing_if = "Option::is_none")]
    pub providers_type: Option<i32>,
    #[serde(rename = "accessLevel", skip_serializing_if = "Option::is_none")]
    pub access_level: Option<i32>,
    #[serde(rename = "providerIds", skip_serializing_if = "Option::is_none")]
    pub provider_ids: Option<Vec<String>>,
    #[serde(rename = "userIds", skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl IntegrationsParams {
    pub fn new() -> IntegrationsParams {
        IntegrationsParams {
            team_ids: None,
            providers_type: None,
            access_level: None,
            provider_ids: None,
            user_ids: None,
        }
    }
}

