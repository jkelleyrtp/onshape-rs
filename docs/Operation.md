# Operation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tags** | Option<**Vec<String>**> |  | [optional]
**summary** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**external_docs** | Option<[**crate::models::ExternalDocumentation**](ExternalDocumentation.md)> |  | [optional]
**operation_id** | Option<**String**> |  | [optional]
**parameters** | Option<[**Vec<crate::models::Parameter>**](Parameter.md)> |  | [optional]
**request_body** | Option<[**crate::models::RequestBody**](RequestBody.md)> |  | [optional]
**responses** | Option<[**::std::collections::HashMap<String, crate::models::ApiResponse>**](ApiResponse.md)> |  | [optional]
**callbacks** | Option<[**::std::collections::HashMap<String, crate::models::Callback>**](Callback.md)> |  | [optional]
**deprecated** | Option<**bool**> |  | [optional]
**security** | Option<[**Vec<crate::models::SecurityRequirement>**](SecurityRequirement.md)> |  | [optional]
**servers** | Option<[**Vec<crate::models::Server>**](Server.md)> |  | [optional]
**extensions** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


