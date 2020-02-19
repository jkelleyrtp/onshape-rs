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
pub struct BtExportTessellatedFacesFacet {
    #[serde(rename = "normal", skip_serializing_if = "Option::is_none")]
    pub normal: Option<crate::models::BtVector3d>,
    #[serde(rename = "indices", skip_serializing_if = "Option::is_none")]
    pub indices: Option<Vec<i32>>,
    #[serde(rename = "normals", skip_serializing_if = "Option::is_none")]
    pub normals: Option<Vec<crate::models::BtVector3d>>,
    #[serde(rename = "vertices", skip_serializing_if = "Option::is_none")]
    pub vertices: Option<Vec<crate::models::BtVector3d>>,
    #[serde(rename = "textureCoordinates", skip_serializing_if = "Option::is_none")]
    pub texture_coordinates: Option<Vec<crate::models::BtVector2d>>,
}

impl BtExportTessellatedFacesFacet {
    pub fn new() -> BtExportTessellatedFacesFacet {
        BtExportTessellatedFacesFacet {
            normal: None,
            indices: None,
            normals: None,
            vertices: None,
            texture_coordinates: None,
        }
    }
}

