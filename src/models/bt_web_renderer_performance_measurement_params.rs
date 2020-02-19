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
pub struct BtWebRendererPerformanceMeasurementParams {
    #[serde(rename = "vendor", skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(rename = "renderer", skip_serializing_if = "Option::is_none")]
    pub renderer: Option<String>,
    #[serde(rename = "trianglesPerSecond", skip_serializing_if = "Option::is_none")]
    pub triangles_per_second: Option<f32>,
    #[serde(rename = "linesPerSecond", skip_serializing_if = "Option::is_none")]
    pub lines_per_second: Option<f32>,
}

impl BtWebRendererPerformanceMeasurementParams {
    pub fn new() -> BtWebRendererPerformanceMeasurementParams {
        BtWebRendererPerformanceMeasurementParams {
            vendor: None,
            renderer: None,
            triangles_per_second: None,
            lines_per_second: None,
        }
    }
}

