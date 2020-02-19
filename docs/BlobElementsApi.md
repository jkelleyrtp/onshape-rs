# \BlobElementsApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_file_workspace**](BlobElementsApi.md#download_file_workspace) | **Get** /api/blobelements/d/{did}/w/{wid}/e/{eid} | 
[**upload_file_create_element**](BlobElementsApi.md#upload_file_create_element) | **Post** /api/blobelements/d/{did}/w/{wid} | 



## download_file_workspace

> download_file_workspace(did, wid, eid, content_disposition, if_none_match, link_document_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**content_disposition** | Option<**String**> |  |  |
**if_none_match** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+octet-stream;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file_create_element

> upload_file_create_element(did, wid, content_disposition, entity, media_type, message_body_workers, parent, providers, body_parts)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**content_disposition** | Option<[**crate::models::ContentDisposition**](ContentDisposition.md)> |  |  |
**entity** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**media_type** | Option<[**crate::models::BodyPartMediaType**](BodyPart_mediaType.md)> |  |  |
**message_body_workers** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**parent** | Option<[**crate::models::MultiPart**](MultiPart.md)> |  |  |
**providers** | Option<[**serde_json::Value**](serde_json::Value.md)> |  |  |
**body_parts** | Option<[**Vec<crate::models::BodyPart>**](crate::models::BodyPart.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

