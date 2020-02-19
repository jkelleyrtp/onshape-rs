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
pub struct BtStandardContentFindUsageParams {
    #[serde(rename = "componentDocumentId", skip_serializing_if = "Option::is_none")]
    pub component_document_id: Option<String>,
    #[serde(rename = "usedFromInDays", skip_serializing_if = "Option::is_none")]
    pub used_from_in_days: Option<String>,
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<crate::models::BtStandardContentParameterDefinition>>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl BtStandardContentFindUsageParams {
    pub fn new() -> BtStandardContentFindUsageParams {
        BtStandardContentFindUsageParams {
            component_document_id: None,
            used_from_in_days: None,
            company_id: None,
            parameters: None,
            user_id: None,
        }
    }
}


