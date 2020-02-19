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
pub struct BtAssemblyTransformedInstancesDefinitionParams {
    #[serde(rename = "transformGroups", skip_serializing_if = "Option::is_none")]
    pub transform_groups: Option<Vec<crate::models::TransformGroup>>,
}

impl BtAssemblyTransformedInstancesDefinitionParams {
    pub fn new() -> BtAssemblyTransformedInstancesDefinitionParams {
        BtAssemblyTransformedInstancesDefinitionParams {
            transform_groups: None,
        }
    }
}

