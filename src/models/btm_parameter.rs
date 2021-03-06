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
pub struct BtmParameter {
    #[serde(rename = "parameterId", skip_serializing_if = "Option::is_none")]
    pub parameter_id: Option<String>,
    #[serde(rename = "importMicroversion", skip_serializing_if = "Option::is_none")]
    pub import_microversion: Option<String>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl BtmParameter {
    pub fn new() -> BtmParameter {
        BtmParameter {
            parameter_id: None,
            import_microversion: None,
            node_id: None,
        }
    }
}


