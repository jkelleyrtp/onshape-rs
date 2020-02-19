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
pub struct BtSeatsParams {
    #[serde(rename = "seats", skip_serializing_if = "Option::is_none")]
    pub seats: Option<i64>,
    #[serde(rename = "lightSeats", skip_serializing_if = "Option::is_none")]
    pub light_seats: Option<i64>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl BtSeatsParams {
    pub fn new() -> BtSeatsParams {
        BtSeatsParams {
            seats: None,
            light_seats: None,
            user_id: None,
        }
    }
}


