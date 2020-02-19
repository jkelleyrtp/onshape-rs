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
pub struct BtIdentityManagementParams {
    #[serde(rename = "providerType", skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<i32>,
    #[serde(rename = "disablePasswordSignIn", skip_serializing_if = "Option::is_none")]
    pub disable_password_sign_in: Option<bool>,
    #[serde(rename = "idpCertificate", skip_serializing_if = "Option::is_none")]
    pub idp_certificate: Option<String>,
    #[serde(rename = "idpEntityId", skip_serializing_if = "Option::is_none")]
    pub idp_entity_id: Option<String>,
    #[serde(rename = "idpRedirectURL", skip_serializing_if = "Option::is_none")]
    pub idp_redirect_url: Option<String>,
    #[serde(rename = "metadataMustBeSigned", skip_serializing_if = "Option::is_none")]
    pub metadata_must_be_signed: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl BtIdentityManagementParams {
    pub fn new() -> BtIdentityManagementParams {
        BtIdentityManagementParams {
            provider_type: None,
            disable_password_sign_in: None,
            idp_certificate: None,
            idp_entity_id: None,
            idp_redirect_url: None,
            metadata_must_be_signed: None,
            name: None,
            enabled: None,
        }
    }
}


