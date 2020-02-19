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
pub struct BtStandardContentCustomParameterDefinition {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::models::BtStandardContentCustomParameterDefinitionId>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(rename = "modifiedAt", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "parameterType", skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<i32>,
    #[serde(rename = "parameterValue", skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    #[serde(rename = "isDrivingConfiguration", skip_serializing_if = "Option::is_none")]
    pub is_driving_configuration: Option<bool>,
    #[serde(rename = "parameterLinkType", skip_serializing_if = "Option::is_none")]
    pub parameter_link_type: Option<i32>,
    #[serde(rename = "new", skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
}

impl BtStandardContentCustomParameterDefinition {
    pub fn new() -> BtStandardContentCustomParameterDefinition {
        BtStandardContentCustomParameterDefinition {
            id: None,
            name: None,
            description: None,
            created_by: None,
            created_at: None,
            modified_by: None,
            modified_at: None,
            parameter_type: None,
            parameter_value: None,
            is_driving_configuration: None,
            parameter_link_type: None,
            new: None,
        }
    }
}

