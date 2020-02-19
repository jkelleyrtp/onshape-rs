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
pub struct BtTranslateFormatParams {
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(rename = "linkDocumentId", skip_serializing_if = "Option::is_none")]
    pub link_document_id: Option<String>,
    #[serde(rename = "linkDocumentWorkspaceId", skip_serializing_if = "Option::is_none")]
    pub link_document_workspace_id: Option<String>,
    #[serde(rename = "storeInDocument", skip_serializing_if = "Option::is_none")]
    pub store_in_document: Option<bool>,
    #[serde(rename = "fromUserId", skip_serializing_if = "Option::is_none")]
    pub from_user_id: Option<String>,
    #[serde(rename = "cloudStorageAccountId", skip_serializing_if = "Option::is_none")]
    pub cloud_storage_account_id: Option<String>,
    #[serde(rename = "connectionId", skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "triggerAutoDownload", skip_serializing_if = "Option::is_none")]
    pub trigger_auto_download: Option<bool>,
    #[serde(rename = "notifyUser", skip_serializing_if = "Option::is_none")]
    pub notify_user: Option<bool>,
    #[serde(rename = "partIds", skip_serializing_if = "Option::is_none")]
    pub part_ids: Option<String>,
    #[serde(rename = "cloudObjectId", skip_serializing_if = "Option::is_none")]
    pub cloud_object_id: Option<String>,
    #[serde(rename = "emailLink", skip_serializing_if = "Option::is_none")]
    pub email_link: Option<bool>,
    #[serde(rename = "passwordRequired", skip_serializing_if = "Option::is_none")]
    pub password_required: Option<bool>,
    #[serde(rename = "emailSubject", skip_serializing_if = "Option::is_none")]
    pub email_subject: Option<String>,
    #[serde(rename = "emailMessage", skip_serializing_if = "Option::is_none")]
    pub email_message: Option<String>,
    #[serde(rename = "emailTo", skip_serializing_if = "Option::is_none")]
    pub email_to: Option<Vec<String>>,
    #[serde(rename = "sendCopyToMe", skip_serializing_if = "Option::is_none")]
    pub send_copy_to_me: Option<bool>,
    #[serde(rename = "versionString", skip_serializing_if = "Option::is_none")]
    pub version_string: Option<String>,
    #[serde(rename = "destinationName", skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,
    #[serde(rename = "sourceName", skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(rename = "distanceTolerance", skip_serializing_if = "Option::is_none")]
    pub distance_tolerance: Option<f64>,
    #[serde(rename = "angularTolerance", skip_serializing_if = "Option::is_none")]
    pub angular_tolerance: Option<f64>,
    #[serde(rename = "maximumChordLength", skip_serializing_if = "Option::is_none")]
    pub maximum_chord_length: Option<f64>,
    #[serde(rename = "validForDays", skip_serializing_if = "Option::is_none")]
    pub valid_for_days: Option<i32>,
    #[serde(rename = "formatName", skip_serializing_if = "Option::is_none")]
    pub format_name: Option<String>,
    #[serde(rename = "flatten", skip_serializing_if = "Option::is_none")]
    pub flatten: Option<bool>,
    #[serde(rename = "includeExportIds", skip_serializing_if = "Option::is_none")]
    pub include_export_ids: Option<bool>,
    #[serde(rename = "textAsGeometry", skip_serializing_if = "Option::is_none")]
    pub text_as_geometry: Option<bool>,
    #[serde(rename = "showOverriddenDimensions", skip_serializing_if = "Option::is_none")]
    pub show_overridden_dimensions: Option<bool>,
    #[serde(rename = "currentSheetOnly", skip_serializing_if = "Option::is_none")]
    pub current_sheet_only: Option<bool>,
    #[serde(rename = "imageWidth", skip_serializing_if = "Option::is_none")]
    pub image_width: Option<i32>,
    #[serde(rename = "imageHeight", skip_serializing_if = "Option::is_none")]
    pub image_height: Option<i32>,
    #[serde(rename = "colorMethod", skip_serializing_if = "Option::is_none")]
    pub color_method: Option<String>,
    #[serde(rename = "splinesAsPolylines", skip_serializing_if = "Option::is_none")]
    pub splines_as_polylines: Option<bool>,
    #[serde(rename = "selectablePdfText", skip_serializing_if = "Option::is_none")]
    pub selectable_pdf_text: Option<bool>,
    #[serde(rename = "grouping", skip_serializing_if = "Option::is_none")]
    pub grouping: Option<bool>,
    #[serde(rename = "importInBackground", skip_serializing_if = "Option::is_none")]
    pub import_in_background: Option<bool>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
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

impl BtTranslateFormatParams {
    pub fn new() -> BtTranslateFormatParams {
        BtTranslateFormatParams {
            configuration: None,
            element_id: None,
            link_document_id: None,
            link_document_workspace_id: None,
            store_in_document: None,
            from_user_id: None,
            cloud_storage_account_id: None,
            connection_id: None,
            trigger_auto_download: None,
            notify_user: None,
            part_ids: None,
            cloud_object_id: None,
            email_link: None,
            password_required: None,
            email_subject: None,
            email_message: None,
            email_to: None,
            send_copy_to_me: None,
            version_string: None,
            destination_name: None,
            source_name: None,
            distance_tolerance: None,
            angular_tolerance: None,
            maximum_chord_length: None,
            valid_for_days: None,
            format_name: None,
            flatten: None,
            include_export_ids: None,
            text_as_geometry: None,
            show_overridden_dimensions: None,
            current_sheet_only: None,
            image_width: None,
            image_height: None,
            color_method: None,
            splines_as_polylines: None,
            selectable_pdf_text: None,
            grouping: None,
            import_in_background: None,
            password: None,
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

