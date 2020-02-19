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
pub struct Coupon {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "amountOff", skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<i64>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "durationInMonths", skip_serializing_if = "Option::is_none")]
    pub duration_in_months: Option<i32>,
    #[serde(rename = "livemode", skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,
    #[serde(rename = "maxRedemptions", skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "percentOff", skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<i32>,
    #[serde(rename = "redeemBy", skip_serializing_if = "Option::is_none")]
    pub redeem_by: Option<i64>,
    #[serde(rename = "timesRedeemed", skip_serializing_if = "Option::is_none")]
    pub times_redeemed: Option<i32>,
    #[serde(rename = "valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}

impl Coupon {
    pub fn new() -> Coupon {
        Coupon {
            id: None,
            object: None,
            amount_off: None,
            created: None,
            currency: None,
            duration: None,
            duration_in_months: None,
            livemode: None,
            max_redemptions: None,
            metadata: None,
            percent_off: None,
            redeem_by: None,
            times_redeemed: None,
            valid: None,
        }
    }
}

