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
pub struct BtVector2d {
    #[serde(rename = "y", skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[serde(rename = "x", skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
}

impl BtVector2d {
    pub fn new() -> BtVector2d {
        BtVector2d {
            y: None,
            x: None,
        }
    }
}

