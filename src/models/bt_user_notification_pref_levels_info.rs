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
pub struct BtUserNotificationPrefLevelsInfo {
    #[serde(rename = "levelsInfoList", skip_serializing_if = "Option::is_none")]
    pub levels_info_list: Option<Vec<crate::models::LevelInfo>>,
}

impl BtUserNotificationPrefLevelsInfo {
    pub fn new() -> BtUserNotificationPrefLevelsInfo {
        BtUserNotificationPrefLevelsInfo {
            levels_info_list: None,
        }
    }
}


