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
pub struct CustomerSubscriptionCollection {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::Subscription>>,
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "hasMore", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(rename = "requestOptions", skip_serializing_if = "Option::is_none")]
    pub request_options: Option<crate::models::RequestOptions>,
    #[serde(rename = "requestParams", skip_serializing_if = "Option::is_none")]
    pub request_params: Option<::std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

impl CustomerSubscriptionCollection {
    pub fn new() -> CustomerSubscriptionCollection {
        CustomerSubscriptionCollection {
            data: None,
            total_count: None,
            has_more: None,
            request_options: None,
            request_params: None,
            url: None,
            count: None,
        }
    }
}


