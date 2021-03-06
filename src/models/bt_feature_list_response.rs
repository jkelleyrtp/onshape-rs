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
pub struct BtFeatureListResponse {
    #[serde(rename = "features", skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<crate::models::BtmFeature>>,
    #[serde(rename = "isComplete", skip_serializing_if = "Option::is_none")]
    pub is_complete: Option<bool>,
    #[serde(rename = "featureStates", skip_serializing_if = "Option::is_none")]
    pub feature_states: Option<::std::collections::HashMap<String, crate::models::BtFeatureState>>,
    #[serde(rename = "defaultFeatures", skip_serializing_if = "Option::is_none")]
    pub default_features: Option<Vec<crate::models::BtmFeature>>,
    #[serde(rename = "imports", skip_serializing_if = "Option::is_none")]
    pub imports: Option<Vec<crate::models::BtmImport>>,
    #[serde(rename = "rollbackIndex", skip_serializing_if = "Option::is_none")]
    pub rollback_index: Option<i32>,
    #[serde(rename = "serializationVersion", skip_serializing_if = "Option::is_none")]
    pub serialization_version: Option<String>,
    #[serde(rename = "libraryVersion", skip_serializing_if = "Option::is_none")]
    pub library_version: Option<i32>,
    #[serde(rename = "sourceMicroversion", skip_serializing_if = "Option::is_none")]
    pub source_microversion: Option<String>,
    #[serde(rename = "rejectMicroversionSkew", skip_serializing_if = "Option::is_none")]
    pub reject_microversion_skew: Option<bool>,
    #[serde(rename = "microversionSkew", skip_serializing_if = "Option::is_none")]
    pub microversion_skew: Option<bool>,
}

impl BtFeatureListResponse {
    pub fn new() -> BtFeatureListResponse {
        BtFeatureListResponse {
            features: None,
            is_complete: None,
            feature_states: None,
            default_features: None,
            imports: None,
            rollback_index: None,
            serialization_version: None,
            library_version: None,
            source_microversion: None,
            reject_microversion_skew: None,
            microversion_skew: None,
        }
    }
}


