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
pub struct BtTraceSamplingRateParams {
    #[serde(rename = "samplingRate", skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<f32>,
}

impl BtTraceSamplingRateParams {
    pub fn new() -> BtTraceSamplingRateParams {
        BtTraceSamplingRateParams {
            sampling_rate: None,
        }
    }
}

