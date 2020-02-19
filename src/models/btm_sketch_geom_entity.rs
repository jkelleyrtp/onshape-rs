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
pub struct BtmSketchGeomEntity {
    #[serde(rename = "controlBoxIds", skip_serializing_if = "Option::is_none")]
    pub control_box_ids: Option<Vec<String>>,
    #[serde(rename = "isConstruction", skip_serializing_if = "Option::is_none")]
    pub is_construction: Option<bool>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::BtmParameter>>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "importMicroversion", skip_serializing_if = "Option::is_none")]
    pub import_microversion: Option<String>,
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}

impl BtmSketchGeomEntity {
    pub fn new() -> BtmSketchGeomEntity {
        BtmSketchGeomEntity {
            control_box_ids: None,
            is_construction: None,
            parameters: None,
            node_id: None,
            namespace: None,
            import_microversion: None,
            entity_id: None,
        }
    }
}


