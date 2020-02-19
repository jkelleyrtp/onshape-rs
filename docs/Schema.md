# Schema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**title** | Option<**String**> |  | [optional]
**multiple_of** | Option<**f32**> |  | [optional]
**maximum** | Option<**f32**> |  | [optional]
**exclusive_maximum** | Option<**bool**> |  | [optional]
**minimum** | Option<**f32**> |  | [optional]
**exclusive_minimum** | Option<**bool**> |  | [optional]
**max_length** | Option<**i32**> |  | [optional]
**min_length** | Option<**i32**> |  | [optional]
**pattern** | Option<**String**> |  | [optional]
**max_items** | Option<**i32**> |  | [optional]
**min_items** | Option<**i32**> |  | [optional]
**unique_items** | Option<**bool**> |  | [optional]
**max_properties** | Option<**i32**> |  | [optional]
**min_properties** | Option<**i32**> |  | [optional]
**required** | Option<**Vec<String>**> |  | [optional]
**_type** | Option<**String**> |  | [optional]
**not** | Option<[**crate::models::Schema**](Schema.md)> |  | [optional]
**properties** | Option<[**::std::collections::HashMap<String, crate::models::Schema>**](Schema.md)> |  | [optional]
**default** | Option<[**serde_json::Value**](.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**format** | Option<**String**> |  | [optional]
**getref** | Option<**String**> |  | [optional]
**nullable** | Option<**bool**> |  | [optional]
**read_only** | Option<**bool**> |  | [optional]
**write_only** | Option<**bool**> |  | [optional]
**external_docs** | Option<[**crate::models::ExternalDocumentation**](ExternalDocumentation.md)> |  | [optional]
**deprecated** | Option<**bool**> |  | [optional]
**xml** | Option<[**crate::models::Xml**](XML.md)> |  | [optional]
**extensions** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**discriminator** | Option<[**crate::models::Discriminator**](Discriminator.md)> |  | [optional]
**_enum** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


