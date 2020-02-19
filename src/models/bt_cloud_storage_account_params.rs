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
pub struct BtCloudStorageAccountParams {
    #[serde(rename = "importFolderId", skip_serializing_if = "Option::is_none")]
    pub import_folder_id: Option<String>,
    #[serde(rename = "exportFolderId", skip_serializing_if = "Option::is_none")]
    pub export_folder_id: Option<String>,
}

impl BtCloudStorageAccountParams {
    pub fn new() -> BtCloudStorageAccountParams {
        BtCloudStorageAccountParams {
            import_folder_id: None,
            export_folder_id: None,
        }
    }
}


