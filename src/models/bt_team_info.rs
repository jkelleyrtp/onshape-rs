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
pub struct BtTeamInfo {
    #[serde(rename = "predefinedTeam", skip_serializing_if = "Option::is_none")]
    pub predefined_team: Option<i32>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "treeHref", skip_serializing_if = "Option::is_none")]
    pub tree_href: Option<String>,
    #[serde(rename = "isMutable", skip_serializing_if = "Option::is_none")]
    pub is_mutable: Option<bool>,
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "modifiedAt", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<crate::models::BtUserBasicSummaryInfo>,
    #[serde(rename = "modifiedBy", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<crate::models::BtUserBasicSummaryInfo>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(rename = "canMove", skip_serializing_if = "Option::is_none")]
    pub can_move: Option<bool>,
    #[serde(rename = "isContainer", skip_serializing_if = "Option::is_none")]
    pub is_container: Option<bool>,
    #[serde(rename = "isEnterpriseOwned", skip_serializing_if = "Option::is_none")]
    pub is_enterprise_owned: Option<bool>,
    #[serde(rename = "hasPendingOwner", skip_serializing_if = "Option::is_none")]
    pub has_pending_owner: Option<bool>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<crate::models::BtOwnerInfo>,
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "viewRef", skip_serializing_if = "Option::is_none")]
    pub view_ref: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(rename = "member", skip_serializing_if = "Option::is_none")]
    pub member: Option<bool>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl BtTeamInfo {
    pub fn new() -> BtTeamInfo {
        BtTeamInfo {
            predefined_team: None,
            active: None,
            tree_href: None,
            is_mutable: None,
            resource_type: None,
            description: None,
            modified_at: None,
            created_at: None,
            created_by: None,
            modified_by: None,
            project_id: None,
            can_move: None,
            is_container: None,
            is_enterprise_owned: None,
            has_pending_owner: None,
            owner: None,
            href: None,
            view_ref: None,
            name: None,
            id: None,
            admin: None,
            member: None,
            size: None,
        }
    }
}


