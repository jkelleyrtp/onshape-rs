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
pub struct BtWebhookOptions {
    #[serde(rename = "collapseEvents", skip_serializing_if = "Option::is_none")]
    pub collapse_events: Option<bool>,
}

impl BtWebhookOptions {
    pub fn new() -> BtWebhookOptions {
        BtWebhookOptions {
            collapse_events: None,
        }
    }
}


