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
pub struct BtesDocumentHit {
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "highlightedFields", skip_serializing_if = "Option::is_none")]
    pub highlighted_fields: Option<::std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "sourceMap", skip_serializing_if = "Option::is_none")]
    pub source_map: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
}

impl BtesDocumentHit {
    pub fn new() -> BtesDocumentHit {
        BtesDocumentHit {
            document_id: None,
            highlighted_fields: None,
            source_map: None,
            name: None,
            _type: None,
        }
    }
}


