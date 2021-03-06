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
pub struct BtVector3d {
    #[serde(rename = "z", skip_serializing_if = "Option::is_none")]
    pub z: Option<f64>,
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
}

impl BtVector3d {
    pub fn new() -> BtVector3d {
        BtVector3d {
            z: None,
            y: None,
            x: None,
        }
    }
}


