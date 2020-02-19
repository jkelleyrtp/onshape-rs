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
pub struct BtRevisionRuleParams {
    #[serde(rename = "revisionList", skip_serializing_if = "Option::is_none")]
    pub revision_list: Option<Vec<String>>,
    #[serde(rename = "ruleType", skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<i32>,
    #[serde(rename = "validationRegex", skip_serializing_if = "Option::is_none")]
    pub validation_regex: Option<String>,
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "script", skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
}

impl BtRevisionRuleParams {
    pub fn new() -> BtRevisionRuleParams {
        BtRevisionRuleParams {
            revision_list: None,
            rule_type: None,
            validation_regex: None,
            company_id: None,
            name: None,
            description: None,
            script: None,
        }
    }
}

