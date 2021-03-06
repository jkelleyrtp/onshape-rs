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
pub struct BtMoveElementParams {
    #[serde(rename = "ownerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "isPublic", skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(rename = "versionName", skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
    #[serde(rename = "targetDocumentId", skip_serializing_if = "Option::is_none")]
    pub target_document_id: Option<String>,
    #[serde(rename = "isCopy", skip_serializing_if = "Option::is_none")]
    pub is_copy: Option<bool>,
    #[serde(rename = "isDeepCopy", skip_serializing_if = "Option::is_none")]
    pub is_deep_copy: Option<bool>,
    #[serde(rename = "isSelectivePartOut", skip_serializing_if = "Option::is_none")]
    pub is_selective_part_out: Option<bool>,
    #[serde(rename = "sourceDocumentId", skip_serializing_if = "Option::is_none")]
    pub source_document_id: Option<String>,
    #[serde(rename = "sourceWorkspaceId", skip_serializing_if = "Option::is_none")]
    pub source_workspace_id: Option<String>,
    #[serde(rename = "ownerType", skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<i32>,
    #[serde(rename = "anchorElementId", skip_serializing_if = "Option::is_none")]
    pub anchor_element_id: Option<String>,
    #[serde(rename = "isGroupAnchor", skip_serializing_if = "Option::is_none")]
    pub is_group_anchor: Option<bool>,
    #[serde(rename = "elementOriginalToNewMap", skip_serializing_if = "Option::is_none")]
    pub element_original_to_new_map: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "targetWorkspaceId", skip_serializing_if = "Option::is_none")]
    pub target_workspace_id: Option<String>,
    #[serde(rename = "generateUnknownMessages", skip_serializing_if = "Option::is_none")]
    pub generate_unknown_messages: Option<bool>,
    #[serde(rename = "importData", skip_serializing_if = "Option::is_none")]
    pub import_data: Option<Vec<String>>,
    #[serde(rename = "needNewVersion", skip_serializing_if = "Option::is_none")]
    pub need_new_version: Option<bool>,
    #[serde(rename = "isNewDocument", skip_serializing_if = "Option::is_none")]
    pub is_new_document: Option<bool>,
    #[serde(rename = "ownerEmail", skip_serializing_if = "Option::is_none")]
    pub owner_email: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "elements", skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<String>>,
}

impl BtMoveElementParams {
    pub fn new() -> BtMoveElementParams {
        BtMoveElementParams {
            owner_id: None,
            project_id: None,
            parent_id: None,
            is_public: None,
            version_name: None,
            target_document_id: None,
            is_copy: None,
            is_deep_copy: None,
            is_selective_part_out: None,
            source_document_id: None,
            source_workspace_id: None,
            owner_type: None,
            anchor_element_id: None,
            is_group_anchor: None,
            element_original_to_new_map: None,
            target_workspace_id: None,
            generate_unknown_messages: None,
            import_data: None,
            need_new_version: None,
            is_new_document: None,
            owner_email: None,
            name: None,
            description: None,
            tags: None,
            elements: None,
        }
    }
}


