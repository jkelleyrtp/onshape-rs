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
pub struct BtNotice {
    #[serde(rename = "locationInfos", skip_serializing_if = "Option::is_none")]
    pub location_infos: Option<Vec<crate::models::BtLocationInfo>>,
    #[serde(rename = "parameterId", skip_serializing_if = "Option::is_none")]
    pub parameter_id: Option<String>,
    #[serde(rename = "tryNode", skip_serializing_if = "Option::is_none")]
    pub try_node: Option<crate::models::BtNodeReference>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "stackTrace", skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<Vec<crate::models::BtLocationInfo>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl BtNotice {
    pub fn new() -> BtNotice {
        BtNotice {
            location_infos: None,
            parameter_id: None,
            try_node: None,
            message: None,
            stack_trace: None,
            _type: None,
            level: None,
            node_id: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "PARSE")]
    PARSE,
    #[serde(rename = "SEMANTIC")]
    SEMANTIC,
    #[serde(rename = "EXECUTION")]
    EXECUTION,
    #[serde(rename = "GEOMETRY")]
    GEOMETRY,
    #[serde(rename = "TEST")]
    TEST,
    #[serde(rename = "OTHER")]
    OTHER,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "INTERNAL")]
    INTERNAL,
    #[serde(rename = "ERROR")]
    ERROR,
    #[serde(rename = "WARNING")]
    WARNING,
    #[serde(rename = "INFO")]
    INFO,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
}
