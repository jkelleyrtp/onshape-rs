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
pub struct BtSurfaceDescription {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
}

impl BtSurfaceDescription {
    pub fn new() -> BtSurfaceDescription {
        BtSurfaceDescription {
            _type: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "PLANE")]
    PLANE,
    #[serde(rename = "CYLINDER")]
    CYLINDER,
    #[serde(rename = "CONE")]
    CONE,
    #[serde(rename = "SPHERE")]
    SPHERE,
    #[serde(rename = "TORUS")]
    TORUS,
    #[serde(rename = "SPUN")]
    SPUN,
    #[serde(rename = "SWEEP")]
    SWEEP,
    #[serde(rename = "OFFSET")]
    OFFSET,
    #[serde(rename = "BLEND")]
    BLEND,
    #[serde(rename = "BSURFACE")]
    BSURFACE,
    #[serde(rename = "OTHER")]
    OTHER,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
}

