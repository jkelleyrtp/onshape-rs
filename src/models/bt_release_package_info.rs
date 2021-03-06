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
pub struct BtReleasePackageInfo {
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::BtReleasePackageItemInfo>>,
    #[serde(rename = "columnNames", skip_serializing_if = "Option::is_none")]
    pub column_names: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "comments", skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<crate::models::BtCommentInfo>>,
    #[serde(rename = "parentComments", skip_serializing_if = "Option::is_none")]
    pub parent_comments: Option<Vec<crate::models::BtReleaseCommentListInfo>>,
    #[serde(rename = "packageThumbnail", skip_serializing_if = "Option::is_none")]
    pub package_thumbnail: Option<String>,
    #[serde(rename = "detailed", skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "parentPackages", skip_serializing_if = "Option::is_none")]
    pub parent_packages: Option<Vec<String>>,
    #[serde(rename = "linkedVersionIds", skip_serializing_if = "Option::is_none")]
    pub linked_version_ids: Option<Vec<String>>,
    #[serde(rename = "revisionRuleId", skip_serializing_if = "Option::is_none")]
    pub revision_rule_id: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "workflow", skip_serializing_if = "Option::is_none")]
    pub workflow: Option<crate::models::BtWorkflowSnapshotInfo>,
    #[serde(rename = "isObsoletion", skip_serializing_if = "Option::is_none")]
    pub is_obsoletion: Option<bool>,
    #[serde(rename = "workflowId", skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<crate::models::BtPublishedWorkflowId>,
    #[serde(rename = "nameAsProperty", skip_serializing_if = "Option::is_none")]
    pub name_as_property: Option<String>,
    #[serde(rename = "descriptionAsProperty", skip_serializing_if = "Option::is_none")]
    pub description_as_property: Option<String>,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<crate::models::BtWorkflowPropertyInfo>>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl BtReleasePackageInfo {
    pub fn new() -> BtReleasePackageInfo {
        BtReleasePackageInfo {
            items: None,
            column_names: None,
            comments: None,
            parent_comments: None,
            package_thumbnail: None,
            detailed: None,
            version_id: None,
            workspace_id: None,
            parent_packages: None,
            linked_version_ids: None,
            revision_rule_id: None,
            description: None,
            company_id: None,
            document_id: None,
            workflow: None,
            is_obsoletion: None,
            workflow_id: None,
            name_as_property: None,
            description_as_property: None,
            properties: None,
            href: None,
            view_ref: None,
            name: None,
            id: None,
        }
    }
}


