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
pub struct FormDataContentDisposition {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "parameters", skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "modificationDate", skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<String>,
    #[serde(rename = "readDate", skip_serializing_if = "Option::is_none")]
    pub read_date: Option<String>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl FormDataContentDisposition {
    pub fn new() -> FormDataContentDisposition {
        FormDataContentDisposition {
            _type: None,
            parameters: None,
            file_name: None,
            creation_date: None,
            modification_date: None,
            read_date: None,
            size: None,
            name: None,
        }
    }
}


