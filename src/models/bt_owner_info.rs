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
pub struct BtOwnerInfo {
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "isEnterpriseOwnedResource", skip_serializing_if = "Option::is_none")]
    pub is_enterprise_owned_resource: Option<bool>,
    #[serde(rename = "acceptOwnershipTransfer", skip_serializing_if = "Option::is_none")]
    pub accept_ownership_transfer: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<i32>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl BtOwnerInfo {
    pub fn new() -> BtOwnerInfo {
        BtOwnerInfo {
            image: None,
            is_enterprise_owned_resource: None,
            accept_ownership_transfer: None,
            _type: None,
            href: None,
            view_ref: None,
            name: None,
            id: None,
        }
    }
}


