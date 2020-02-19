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
pub struct BtCommentInfo {
    #[serde(rename = "elementQuery", skip_serializing_if = "Option::is_none")]
    pub element_query: Option<String>,
    #[serde(rename = "elementFeature", skip_serializing_if = "Option::is_none")]
    pub element_feature: Option<String>,
    #[serde(rename = "assignee", skip_serializing_if = "Option::is_none")]
    pub assignee: Option<crate::models::BtUserSummaryInfo>,
    #[serde(rename = "assignedAt", skip_serializing_if = "Option::is_none")]
    pub assigned_at: Option<String>,
    #[serde(rename = "viewData", skip_serializing_if = "Option::is_none")]
    pub view_data: Option<crate::models::BtViewDataInfo>,
    #[serde(rename = "topLevel", skip_serializing_if = "Option::is_none")]
    pub top_level: Option<bool>,
    #[serde(rename = "reopenedBy", skip_serializing_if = "Option::is_none")]
    pub reopened_by: Option<crate::models::BtUserSummaryInfo>,
    #[serde(rename = "reopenedAt", skip_serializing_if = "Option::is_none")]
    pub reopened_at: Option<String>,
    #[serde(rename = "attachment", skip_serializing_if = "Option::is_none")]
    pub attachment: Option<crate::models::BtCommentAttachmentInfo>,
    #[serde(rename = "resolvedAt", skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::BtUserSummaryInfo>,
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "canDelete", skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    #[serde(rename = "resolvedBy", skip_serializing_if = "Option::is_none")]
    pub resolved_by: Option<crate::models::BtUserSummaryInfo>,
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "thumbnail", skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<crate::models::BtCommentAttachmentInfo>,
    #[serde(rename = "releasePackageId", skip_serializing_if = "Option::is_none")]
    pub release_package_id: Option<String>,
    #[serde(rename = "elementOccurrences", skip_serializing_if = "Option::is_none")]
    pub element_occurrences: Option<Vec<String>>,
    #[serde(rename = "assemblyFeatures", skip_serializing_if = "Option::is_none")]
    pub assembly_features: Option<Vec<String>>,
    #[serde(rename = "canResolveOrReopen", skip_serializing_if = "Option::is_none")]
    pub can_resolve_or_reopen: Option<bool>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl BtCommentInfo {
    pub fn new() -> BtCommentInfo {
        BtCommentInfo {
            element_query: None,
            element_feature: None,
            assignee: None,
            assigned_at: None,
            view_data: None,
            top_level: None,
            reopened_by: None,
            reopened_at: None,
            attachment: None,
            resolved_at: None,
            version_id: None,
            created_at: None,
            workspace_id: None,
            element_id: None,
            user: None,
            document_id: None,
            can_delete: None,
            resolved_by: None,
            parent_id: None,
            thumbnail: None,
            release_package_id: None,
            element_occurrences: None,
            assembly_features: None,
            can_resolve_or_reopen: None,
            message: None,
            state: None,
            href: None,
            view_ref: None,
            name: None,
            id: None,
        }
    }
}

