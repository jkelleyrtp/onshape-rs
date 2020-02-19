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
pub struct BtQueryRevisionParams {
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(rename = "nodeIds", skip_serializing_if = "Option::is_none")]
    pub node_ids: Option<Vec<String>>,
}

impl BtQueryRevisionParams {
    pub fn new() -> BtQueryRevisionParams {
        BtQueryRevisionParams {
            company_id: None,
            node_ids: None,
        }
    }
}


