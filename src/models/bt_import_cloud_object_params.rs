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
pub struct BtImportCloudObjectParams {
    #[serde(rename = "cloudStorageAccountId", skip_serializing_if = "Option::is_none")]
    pub cloud_storage_account_id: Option<String>,
    #[serde(rename = "cloudStorageProvider", skip_serializing_if = "Option::is_none")]
    pub cloud_storage_provider: Option<i32>,
    #[serde(rename = "ownerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "notifyUser", skip_serializing_if = "Option::is_none")]
    pub notify_user: Option<bool>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "cloudObjectId", skip_serializing_if = "Option::is_none")]
    pub cloud_object_id: Option<String>,
    #[serde(rename = "sizeBytes", skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    #[serde(rename = "accessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "ownerType", skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<OwnerType>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "foreignId", skip_serializing_if = "Option::is_none")]
    pub foreign_id: Option<String>,
    #[serde(rename = "uploadId", skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
    #[serde(rename = "unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "specifyUnits", skip_serializing_if = "Option::is_none")]
    pub specify_units: Option<bool>,
    #[serde(rename = "splitAssembliesIntoMultipleDocuments", skip_serializing_if = "Option::is_none")]
    pub split_assemblies_into_multiple_documents: Option<bool>,
    #[serde(rename = "flattenAssemblies", skip_serializing_if = "Option::is_none")]
    pub flatten_assemblies: Option<bool>,
    #[serde(rename = "getyAxisIsUp", skip_serializing_if = "Option::is_none")]
    pub gety_axis_is_up: Option<bool>,
    #[serde(rename = "allowFaultyParts", skip_serializing_if = "Option::is_none")]
    pub allow_faulty_parts: Option<bool>,
    #[serde(rename = "createComposite", skip_serializing_if = "Option::is_none")]
    pub create_composite: Option<bool>,
    #[serde(rename = "extractAssemblyHierarchy", skip_serializing_if = "Option::is_none")]
    pub extract_assembly_hierarchy: Option<bool>,
    #[serde(rename = "joinAdjacentSurfaces", skip_serializing_if = "Option::is_none")]
    pub join_adjacent_surfaces: Option<bool>,
    #[serde(rename = "processedForeignId", skip_serializing_if = "Option::is_none")]
    pub processed_foreign_id: Option<String>,
    #[serde(rename = "originalForeignId", skip_serializing_if = "Option::is_none")]
    pub original_foreign_id: Option<String>,
    #[serde(rename = "importWithinDocument", skip_serializing_if = "Option::is_none")]
    pub import_within_document: Option<bool>,
    #[serde(rename = "blobElementId", skip_serializing_if = "Option::is_none")]
    pub blob_element_id: Option<String>,
    #[serde(rename = "blobMicroversionId", skip_serializing_if = "Option::is_none")]
    pub blob_microversion_id: Option<String>,
}

impl BtImportCloudObjectParams {
    pub fn new() -> BtImportCloudObjectParams {
        BtImportCloudObjectParams {
            cloud_storage_account_id: None,
            cloud_storage_provider: None,
            owner_id: None,
            notify_user: None,
            url: None,
            cloud_object_id: None,
            size_bytes: None,
            access_token: None,
            name: None,
            public: None,
            mime_type: None,
            owner_type: None,
            project_id: None,
            parent_id: None,
            foreign_id: None,
            upload_id: None,
            unit: None,
            specify_units: None,
            split_assemblies_into_multiple_documents: None,
            flatten_assemblies: None,
            gety_axis_is_up: None,
            allow_faulty_parts: None,
            create_composite: None,
            extract_assembly_hierarchy: None,
            join_adjacent_surfaces: None,
            processed_foreign_id: None,
            original_foreign_id: None,
            import_within_document: None,
            blob_element_id: None,
            blob_microversion_id: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OwnerType {
    #[serde(rename = "USER")]
    USER,
    #[serde(rename = "COMPANY")]
    COMPANY,
    #[serde(rename = "ONSHAPE")]
    ONSHAPE,
}
