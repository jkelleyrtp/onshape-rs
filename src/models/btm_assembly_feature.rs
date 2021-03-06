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
pub struct BtmAssemblyFeature {
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "importMicroversion", skip_serializing_if = "Option::is_none")]
    pub import_microversion: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "suppressed", skip_serializing_if = "Option::is_none")]
    pub suppressed: Option<bool>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::BtmParameter>>,
    #[serde(rename = "featureId", skip_serializing_if = "Option::is_none")]
    pub feature_id: Option<String>,
    #[serde(rename = "featureType", skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<String>,
    #[serde(rename = "returnAfterSubfeatures", skip_serializing_if = "Option::is_none")]
    pub return_after_subfeatures: Option<bool>,
    #[serde(rename = "occurrenceQueriesFromAllConfigurations", skip_serializing_if = "Option::is_none")]
    pub occurrence_queries_from_all_configurations: Option<Vec<crate::models::BtmIndividualQueryWithOccurrenceBase>>,
    #[serde(rename = "featureListFieldIndex", skip_serializing_if = "Option::is_none")]
    pub feature_list_field_index: Option<i32>,
    #[serde(rename = "auxiliaryAssemblyFeature", skip_serializing_if = "Option::is_none")]
    pub auxiliary_assembly_feature: Option<bool>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

impl BtmAssemblyFeature {
    pub fn new() -> BtmAssemblyFeature {
        BtmAssemblyFeature {
            node_id: None,
            namespace: None,
            import_microversion: None,
            name: None,
            suppressed: None,
            parameters: None,
            feature_id: None,
            feature_type: None,
            return_after_subfeatures: None,
            occurrence_queries_from_all_configurations: None,
            feature_list_field_index: None,
            auxiliary_assembly_feature: None,
            version: None,
        }
    }
}


