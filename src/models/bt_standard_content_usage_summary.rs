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
pub struct BtStandardContentUsageSummary {
    #[serde(rename = "workspaceUseCount", skip_serializing_if = "Option::is_none")]
    pub workspace_use_count: Option<i64>,
    #[serde(rename = "topUsedComponentsByWorkspaces", skip_serializing_if = "Option::is_none")]
    pub top_used_components_by_workspaces: Option<::std::collections::HashMap<String, crate::models::BtComponentUsagesSummary>>,
    #[serde(rename = "latestVersionUseCount", skip_serializing_if = "Option::is_none")]
    pub latest_version_use_count: Option<i64>,
    #[serde(rename = "topUsedComponentsByLatestVersions", skip_serializing_if = "Option::is_none")]
    pub top_used_components_by_latest_versions: Option<::std::collections::HashMap<String, crate::models::BtComponentUsagesSummary>>,
    #[serde(rename = "versionUseCount", skip_serializing_if = "Option::is_none")]
    pub version_use_count: Option<i64>,
    #[serde(rename = "topComponentsUsedAlongWithByLatestVersions", skip_serializing_if = "Option::is_none")]
    pub top_components_used_along_with_by_latest_versions: Option<::std::collections::HashMap<String, crate::models::BtComponentUsagesSummary>>,
    #[serde(rename = "topUsersByWorkspaces", skip_serializing_if = "Option::is_none")]
    pub top_users_by_workspaces: Option<::std::collections::HashMap<String, crate::models::BtUserUsagesSummary>>,
    #[serde(rename = "topUsersByLatestVersions", skip_serializing_if = "Option::is_none")]
    pub top_users_by_latest_versions: Option<::std::collections::HashMap<String, crate::models::BtUserUsagesSummary>>,
    #[serde(rename = "topUsersSummaryDone", skip_serializing_if = "Option::is_none")]
    pub top_users_summary_done: Option<bool>,
}

impl BtStandardContentUsageSummary {
    pub fn new() -> BtStandardContentUsageSummary {
        BtStandardContentUsageSummary {
            workspace_use_count: None,
            top_used_components_by_workspaces: None,
            latest_version_use_count: None,
            top_used_components_by_latest_versions: None,
            version_use_count: None,
            top_components_used_along_with_by_latest_versions: None,
            top_users_by_workspaces: None,
            top_users_by_latest_versions: None,
            top_users_summary_done: None,
        }
    }
}


