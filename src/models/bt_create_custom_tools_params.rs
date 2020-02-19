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
pub struct BtCreateCustomToolsParams {
    #[serde(rename = "specsExist_", skip_serializing_if = "Option::is_none")]
    pub specs_exist_: Option<bool>,
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "tools", skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<crate::models::BtCreateCustomToolParams>>,
    #[serde(rename = "specsExist", skip_serializing_if = "Option::is_none")]
    pub specs_exist: Option<bool>,
}

impl BtCreateCustomToolsParams {
    pub fn new() -> BtCreateCustomToolsParams {
        BtCreateCustomToolsParams {
            specs_exist_: None,
            parent_id: None,
            tools: None,
            specs_exist: None,
        }
    }
}


