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
pub struct BtSurveyResponseParams {
    #[serde(rename = "msgId", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    #[serde(rename = "surveyType", skip_serializing_if = "Option::is_none")]
    pub survey_type: Option<i32>,
    #[serde(rename = "propertyMap", skip_serializing_if = "Option::is_none")]
    pub property_map: Option<::std::collections::HashMap<String, serde_json::Value>>,
}

impl BtSurveyResponseParams {
    pub fn new() -> BtSurveyResponseParams {
        BtSurveyResponseParams {
            msg_id: None,
            survey_type: None,
            property_map: None,
        }
    }
}


