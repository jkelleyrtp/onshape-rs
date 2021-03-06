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
pub struct BtCampaignParams {
    #[serde(rename = "campaignName", skip_serializing_if = "Option::is_none")]
    pub campaign_name: Option<String>,
    #[serde(rename = "messages", skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<crate::models::Message>>,
    #[serde(rename = "terminated", skip_serializing_if = "Option::is_none")]
    pub terminated: Option<bool>,
}

impl BtCampaignParams {
    pub fn new() -> BtCampaignParams {
        BtCampaignParams {
            campaign_name: None,
            messages: None,
            terminated: None,
        }
    }
}


