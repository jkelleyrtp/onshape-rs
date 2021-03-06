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
pub struct BtUniqueDocumentItemParams {
    #[serde(rename = "versionId", skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "partNumber", skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "partId", skip_serializing_if = "Option::is_none")]
    pub part_id: Option<String>,
    #[serde(rename = "apiConfiguration", skip_serializing_if = "Option::is_none")]
    pub api_configuration: Option<String>,
    #[serde(rename = "revision", skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(rename = "elementType", skip_serializing_if = "Option::is_none")]
    pub element_type: Option<String>,
}

impl BtUniqueDocumentItemParams {
    pub fn new() -> BtUniqueDocumentItemParams {
        BtUniqueDocumentItemParams {
            version_id: None,
            workspace_id: None,
            element_id: None,
            document_id: None,
            part_number: None,
            part_id: None,
            api_configuration: None,
            revision: None,
            element_type: None,
        }
    }
}


