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
pub struct BtStandardContentSetRunVersionsParams {
    #[serde(rename = "componentDocumentId", skip_serializing_if = "Option::is_none")]
    pub component_document_id: Option<String>,
    #[serde(rename = "productionVersionId", skip_serializing_if = "Option::is_none")]
    pub production_version_id: Option<String>,
    #[serde(rename = "testVersionId", skip_serializing_if = "Option::is_none")]
    pub test_version_id: Option<String>,
}

impl BtStandardContentSetRunVersionsParams {
    pub fn new() -> BtStandardContentSetRunVersionsParams {
        BtStandardContentSetRunVersionsParams {
            component_document_id: None,
            production_version_id: None,
            test_version_id: None,
        }
    }
}


