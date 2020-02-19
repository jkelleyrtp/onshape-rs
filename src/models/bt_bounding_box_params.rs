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
pub struct BtBoundingBoxParams {
    #[serde(rename = "includeHidden", skip_serializing_if = "Option::is_none")]
    pub include_hidden: Option<bool>,
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "elementId", skip_serializing_if = "Option::is_none")]
    pub element_id: Option<String>,
    #[serde(rename = "documentId", skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    #[serde(rename = "partQuery", skip_serializing_if = "Option::is_none")]
    pub part_query: Option<String>,
    #[serde(rename = "partId", skip_serializing_if = "Option::is_none")]
    pub part_id: Option<String>,
    #[serde(rename = "includeWireBodies", skip_serializing_if = "Option::is_none")]
    pub include_wire_bodies: Option<bool>,
    #[serde(rename = "displayStateId", skip_serializing_if = "Option::is_none")]
    pub display_state_id: Option<String>,
    #[serde(rename = "explosionId", skip_serializing_if = "Option::is_none")]
    pub explosion_id: Option<String>,
    #[serde(rename = "elementMicroversionId", skip_serializing_if = "Option::is_none")]
    pub element_microversion_id: Option<String>,
    #[serde(rename = "sketchId", skip_serializing_if = "Option::is_none")]
    pub sketch_id: Option<String>,
}

impl BtBoundingBoxParams {
    pub fn new() -> BtBoundingBoxParams {
        BtBoundingBoxParams {
            include_hidden: None,
            workspace_id: None,
            element_id: None,
            document_id: None,
            part_query: None,
            part_id: None,
            include_wire_bodies: None,
            display_state_id: None,
            explosion_id: None,
            element_microversion_id: None,
            sketch_id: None,
        }
    }
}

