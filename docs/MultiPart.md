# MultiPart

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_disposition** | Option<[**crate::models::ContentDisposition**](ContentDisposition.md)> |  | [optional]
**entity** | Option<[**serde_json::Value**](.md)> |  | [optional]
**headers** | Option<[**::std::collections::HashMap<String, Vec<String>>**](array.md)> |  | [optional]
**media_type** | Option<[**crate::models::BodyPartMediaType**](BodyPart_mediaType.md)> |  | [optional]
**message_body_workers** | Option<[**serde_json::Value**](.md)> |  | [optional]
**parent** | Option<[**crate::models::MultiPart**](MultiPart.md)> |  | [optional]
**providers** | Option<[**serde_json::Value**](.md)> |  | [optional]
**body_parts** | Option<[**Vec<crate::models::BodyPart>**](BodyPart.md)> |  | [optional]
**parameterized_headers** | Option<[**::std::collections::HashMap<String, Vec<crate::models::ParameterizedHeader>>**](array.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


