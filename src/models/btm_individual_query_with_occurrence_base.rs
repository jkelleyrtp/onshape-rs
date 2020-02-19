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
pub struct BtmIndividualQueryWithOccurrenceBase {
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<String>>,
    #[serde(rename = "queryString", skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(rename = "deterministicIds", skip_serializing_if = "Option::is_none")]
    pub deterministic_ids: Option<Vec<String>>,
    #[serde(rename = "deterministicIdList", skip_serializing_if = "Option::is_none")]
    pub deterministic_id_list: Option<crate::models::BtmIndividualQueryBase>,
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<crate::models::BtmIndividualQueryBase>,
    #[serde(rename = "importMicroversion", skip_serializing_if = "Option::is_none")]
    pub import_microversion: Option<String>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl BtmIndividualQueryWithOccurrenceBase {
    pub fn new() -> BtmIndividualQueryWithOccurrenceBase {
        BtmIndividualQueryWithOccurrenceBase {
            path: None,
            query_string: None,
            deterministic_ids: None,
            deterministic_id_list: None,
            query: None,
            import_microversion: None,
            node_id: None,
        }
    }
}

