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
pub struct BtapiApplicationDeprecateParams {
    #[serde(rename = "ownerEmailSubjectText", skip_serializing_if = "Option::is_none")]
    pub owner_email_subject_text: Option<String>,
    #[serde(rename = "ownerEmailMessageText", skip_serializing_if = "Option::is_none")]
    pub owner_email_message_text: Option<String>,
    #[serde(rename = "userEmailSubjectText", skip_serializing_if = "Option::is_none")]
    pub user_email_subject_text: Option<String>,
    #[serde(rename = "userEmailMessageText", skip_serializing_if = "Option::is_none")]
    pub user_email_message_text: Option<String>,
}

impl BtapiApplicationDeprecateParams {
    pub fn new() -> BtapiApplicationDeprecateParams {
        BtapiApplicationDeprecateParams {
            owner_email_subject_text: None,
            owner_email_message_text: None,
            user_email_subject_text: None,
            user_email_message_text: None,
        }
    }
}


