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
pub struct BtRefreshReleaseParams {
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "createNew", skip_serializing_if = "Option::is_none")]
    pub create_new: Option<bool>,
    #[serde(rename = "updateParams", skip_serializing_if = "Option::is_none")]
    pub update_params: Option<crate::models::BtUpdateReleasePackageParams>,
    #[serde(rename = "workflowId", skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

impl BtRefreshReleaseParams {
    pub fn new() -> BtRefreshReleaseParams {
        BtRefreshReleaseParams {
            workspace_id: None,
            create_new: None,
            update_params: None,
            workflow_id: None,
        }
    }
}

