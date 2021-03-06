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
pub struct BtDiscountInfo {
    #[serde(rename = "percentOff", skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<i32>,
    #[serde(rename = "amountOff", skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<i32>,
    #[serde(rename = "couponType", skip_serializing_if = "Option::is_none")]
    pub coupon_type: Option<i32>,
    #[serde(rename = "couponValidMonths", skip_serializing_if = "Option::is_none")]
    pub coupon_valid_months: Option<i32>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "planId", skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<crate::models::BtUserSummaryInfo>,
    #[serde(rename = "ownerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "usedAt", skip_serializing_if = "Option::is_none")]
    pub used_at: Option<String>,
    #[serde(rename = "trialEndDate", skip_serializing_if = "Option::is_none")]
    pub trial_end_date: Option<String>,
    #[serde(rename = "accountBalance", skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<i32>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl BtDiscountInfo {
    pub fn new() -> BtDiscountInfo {
        BtDiscountInfo {
            percent_off: None,
            amount_off: None,
            coupon_type: None,
            coupon_valid_months: None,
            expires_at: None,
            created_at: None,
            plan_id: None,
            created_by: None,
            owner_id: None,
            used_at: None,
            trial_end_date: None,
            account_balance: None,
            href: None,
            view_ref: None,
            name: None,
            id: None,
        }
    }
}


