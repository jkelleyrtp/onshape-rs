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
pub struct BtBillingAccountInfo {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::models::BtAddressInfo>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "card", skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::models::BtCardInfo>,
    #[serde(rename = "accountBalance", skip_serializing_if = "Option::is_none")]
    pub account_balance: Option<i64>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    #[serde(rename = "company", skip_serializing_if = "Option::is_none")]
    pub company: Option<crate::models::BtCompanyInfo>,
    #[serde(rename = "stripeId", skip_serializing_if = "Option::is_none")]
    pub stripe_id: Option<String>,
    #[serde(rename = "proMonthlyEnabled", skip_serializing_if = "Option::is_none")]
    pub pro_monthly_enabled: Option<bool>,
    #[serde(rename = "zuoraId", skip_serializing_if = "Option::is_none")]
    pub zuora_id: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<crate::models::BtUserSummaryInfo>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl BtBillingAccountInfo {
    pub fn new() -> BtBillingAccountInfo {
        BtBillingAccountInfo {
            address: None,
            created_at: None,
            card: None,
            account_balance: None,
            state: None,
            company: None,
            stripe_id: None,
            pro_monthly_enabled: None,
            zuora_id: None,
            owner: None,
            href: None,
            view_ref: None,
            name: None,
            id: None,
        }
    }
}


