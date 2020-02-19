# OpenApi

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**openapi** | Option<**String**> |  | [optional]
**info** | Option<[**crate::models::Info**](Info.md)> |  | [optional]
**external_docs** | Option<[**crate::models::ExternalDocumentation**](ExternalDocumentation.md)> |  | [optional]
**servers** | Option<[**Vec<crate::models::Server>**](Server.md)> |  | [optional]
**security** | Option<[**Vec<crate::models::SecurityRequirement>**](SecurityRequirement.md)> |  | [optional]
**tags** | Option<[**Vec<crate::models::Tag>**](Tag.md)> |  | [optional]
**paths** | Option<[**::std::collections::HashMap<String, crate::models::PathItem>**](PathItem.md)> |  | [optional]
**components** | Option<[**crate::models::Components**](Components.md)> |  | [optional]
**extensions** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


