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
pub struct Plan {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(rename = "intervalCount", skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<i32>,
    #[serde(rename = "livemode", skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "statementDescriptor", skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(rename = "trialPeriodDays", skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i32>,
    #[serde(rename = "statementDescription", skip_serializing_if = "Option::is_none")]
    pub statement_description: Option<String>,
}

impl Plan {
    pub fn new() -> Plan {
        Plan {
            id: None,
            object: None,
            amount: None,
            created: None,
            currency: None,
            interval: None,
            interval_count: None,
            livemode: None,
            metadata: None,
            name: None,
            statement_descriptor: None,
            trial_period_days: None,
            statement_description: None,
        }
    }
}


