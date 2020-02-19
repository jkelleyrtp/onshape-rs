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
pub struct BtGlobalTreeNodeOwnerParams {
    #[serde(rename = "newOwnerId", skip_serializing_if = "Option::is_none")]
    pub new_owner_id: Option<String>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "newOwnerType", skip_serializing_if = "Option::is_none")]
    pub new_owner_type: Option<i32>,
    #[serde(rename = "personalMessage", skip_serializing_if = "Option::is_none")]
    pub personal_message: Option<String>,
    #[serde(rename = "newUserEmail", skip_serializing_if = "Option::is_none")]
    pub new_user_email: Option<String>,
}

impl BtGlobalTreeNodeOwnerParams {
    pub fn new() -> BtGlobalTreeNodeOwnerParams {
        BtGlobalTreeNodeOwnerParams {
            new_owner_id: None,
            resource_id: None,
            resource_type: None,
            new_owner_type: None,
            personal_message: None,
            new_user_email: None,
        }
    }
}

