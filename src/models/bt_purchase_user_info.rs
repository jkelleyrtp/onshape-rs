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
pub struct BtPurchaseUserInfo {
    #[serde(rename = "purchase", skip_serializing_if = "Option::is_none")]
    pub purchase: Option<crate::models::BtPurchaseInfo>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::models::BtUserBasicSummaryInfo>,
    #[serde(rename = "consumedQuantity", skip_serializing_if = "Option::is_none")]
    pub consumed_quantity: Option<i32>,
    #[serde(rename = "purchaseState", skip_serializing_if = "Option::is_none")]
    pub purchase_state: Option<i32>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl BtPurchaseUserInfo {
    pub fn new() -> BtPurchaseUserInfo {
        BtPurchaseUserInfo {
            purchase: None,
            user: None,
            consumed_quantity: None,
            purchase_state: None,
            href: None,
            view_ref: None,
            name: None,
            id: None,
        }
    }
}


