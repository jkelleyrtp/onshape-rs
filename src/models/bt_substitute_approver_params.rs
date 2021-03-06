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
pub struct BtSubstituteApproverParams {
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<crate::models::BtIdentityParams>,
    #[serde(rename = "companyId", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl BtSubstituteApproverParams {
    pub fn new() -> BtSubstituteApproverParams {
        BtSubstituteApproverParams {
            identity: None,
            company_id: None,
            enabled: None,
        }
    }
}


