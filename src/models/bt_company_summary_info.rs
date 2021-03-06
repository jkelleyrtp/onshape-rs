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
pub struct BtCompanySummaryInfo {
    #[serde(rename = "domainPrefix", skip_serializing_if = "Option::is_none")]
    pub domain_prefix: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(rename = "ownerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "enterpriseBaseUrl", skip_serializing_if = "Option::is_none")]
    pub enterprise_base_url: Option<String>,
    #[serde(rename = "noPublicDocuments", skip_serializing_if = "Option::is_none")]
    pub no_public_documents: Option<bool>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
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

impl BtCompanySummaryInfo {
    pub fn new() -> BtCompanySummaryInfo {
        BtCompanySummaryInfo {
            domain_prefix: None,
            description: None,
            admin: None,
            owner_id: None,
            image: None,
            enterprise_base_url: None,
            no_public_documents: None,
            state: None,
            _type: None,
            href: None,
            view_ref: None,
            name: None,
            id: None,
        }
    }
}


