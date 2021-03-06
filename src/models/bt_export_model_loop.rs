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
pub struct BtExportModelLoop {
    #[serde(rename = "isOuter", skip_serializing_if = "Option::is_none")]
    pub is_outer: Option<bool>,
    #[serde(rename = "isInner", skip_serializing_if = "Option::is_none")]
    pub is_inner: Option<bool>,
    #[serde(rename = "coedges", skip_serializing_if = "Option::is_none")]
    pub coedges: Option<Vec<crate::models::BtExportModelCoedge>>,
}

impl BtExportModelLoop {
    pub fn new() -> BtExportModelLoop {
        BtExportModelLoop {
            is_outer: None,
            is_inner: None,
            coedges: None,
        }
    }
}


