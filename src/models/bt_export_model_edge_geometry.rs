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
pub struct BtExportModelEdgeGeometry {
    #[serde(rename = "midPoint", skip_serializing_if = "Option::is_none")]
    pub mid_point: Option<crate::models::BtVector3d>,
    #[serde(rename = "quarterPoint", skip_serializing_if = "Option::is_none")]
    pub quarter_point: Option<crate::models::BtVector3d>,
    #[serde(rename = "startVector", skip_serializing_if = "Option::is_none")]
    pub start_vector: Option<crate::models::BtVector3d>,
    #[serde(rename = "endVector", skip_serializing_if = "Option::is_none")]
    pub end_vector: Option<crate::models::BtVector3d>,
    #[serde(rename = "startPoint", skip_serializing_if = "Option::is_none")]
    pub start_point: Option<crate::models::BtVector3d>,
    #[serde(rename = "endPoint", skip_serializing_if = "Option::is_none")]
    pub end_point: Option<crate::models::BtVector3d>,
    #[serde(rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
}

impl BtExportModelEdgeGeometry {
    pub fn new() -> BtExportModelEdgeGeometry {
        BtExportModelEdgeGeometry {
            mid_point: None,
            quarter_point: None,
            start_vector: None,
            end_vector: None,
            start_point: None,
            end_point: None,
            length: None,
        }
    }
}

