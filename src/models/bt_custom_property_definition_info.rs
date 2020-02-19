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
pub struct BtCustomPropertyDefinitionInfo {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "template", skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
    #[serde(rename = "enumDefinition", skip_serializing_if = "Option::is_none")]
    pub enum_definition: Option<Vec<String>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
}

impl BtCustomPropertyDefinitionInfo {
    pub fn new() -> BtCustomPropertyDefinitionInfo {
        BtCustomPropertyDefinitionInfo {
            description: None,
            template: None,
            enum_definition: None,
            name: None,
            _type: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "STRING")]
    STRING,
    #[serde(rename = "NUMBER")]
    NUMBER,
    #[serde(rename = "INTEGER")]
    INTEGER,
    #[serde(rename = "BOOLEAN")]
    BOOLEAN,
    #[serde(rename = "DATE")]
    DATE,
    #[serde(rename = "ENUM")]
    _ENUM,
    #[serde(rename = "BLOB")]
    BLOB,
    #[serde(rename = "OBJECT")]
    OBJECT,
    #[serde(rename = "ARRAY")]
    ARRAY,
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
}
