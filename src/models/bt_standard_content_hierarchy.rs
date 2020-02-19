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
pub struct BtStandardContentHierarchy {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "standard", skip_serializing_if = "Option::is_none")]
    pub standard: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "types", skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "documentType", skip_serializing_if = "Option::is_none")]
    pub document_type: Option<i32>,
    #[serde(rename = "productionVersionId", skip_serializing_if = "Option::is_none")]
    pub production_version_id: Option<String>,
    #[serde(rename = "testVersionId", skip_serializing_if = "Option::is_none")]
    pub test_version_id: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "modifiedAt", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
}

impl BtStandardContentHierarchy {
    pub fn new() -> BtStandardContentHierarchy {
        BtStandardContentHierarchy {
            id: None,
            standard: None,
            category: None,
            types: None,
            _type: None,
            document_type: None,
            production_version_id: None,
            test_version_id: None,
            created_at: None,
            modified_at: None,
            created_by: None,
            modified_by: None,
            document_id: None,
        }
    }
}

