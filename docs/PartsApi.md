# \PartsApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_parts_wmv**](PartsApi.md#get_parts_wmv) | **Get** /api/parts/d/{did}/{wvm}/{wvmid} | Get list of parts
[**get_parts_wmve**](PartsApi.md#get_parts_wmve) | **Get** /api/parts/d/{did}/{wvm}/{wvmid}/e/{eid} | Get parts from an element.
[**update_parts_wmv**](PartsApi.md#update_parts_wmv) | **Post** /api/parts/d/{did}/{wvm}/{wvmid} | Part metadata batch update.



## get_parts_wmv

> Vec<crate::models::BtPartMetadataInfo> get_parts_wmv(did, wvm, wvmid, element_id, with_thumbnails, include_property_defaults, link_document_id, configuration)
Get list of parts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**element_id** | Option<**String**> | Element ID |  |
**with_thumbnails** | Option<**bool**> | Whether or not to include thumbnails (not supported for microversion) |  |[default to false]
**include_property_defaults** | Option<**bool**> | If true, include metadata schema property defaults in response |  |[default to false]
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |
**configuration** | Option<**String**> | Configuration string. |  |

### Return type

[**Vec<crate::models::BtPartMetadataInfo>**](BTPartMetadataInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_parts_wmve

> Vec<crate::models::BtPartMetadataInfo> get_parts_wmve(did, wvm, wvmid, eid, with_thumbnails, include_property_defaults, configuration, link_document_id)
Get parts from an element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**eid** | **String** | Element ID. | [required] |
**with_thumbnails** | Option<**bool**> | Whether or not to include thumbnails (not supported for microversion) |  |[default to false]
**include_property_defaults** | Option<**bool**> | If true, include metadata schema property defaults in response |  |[default to false]
**configuration** | Option<**String**> | Configuration string. |  |
**link_document_id** | Option<**String**> | Id of document that links to the document being accessed. This may provide additional access rights to the document. Allowed only with version (v) path parameter. |  |

### Return type

[**Vec<crate::models::BtPartMetadataInfo>**](BTPartMetadataInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_parts_wmv

> update_parts_wmv(did, wvm, wvmid, edit_description, body)
Part metadata batch update.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** | Document ID. | [required] |
**wvm** | **String** | One of w or v or m corresponding to whether a workspace or version or microversion was entered. | [required] |
**wvmid** | **String** | Workspace (w), Version (v) or Microversion (m) ID. | [required] |
**edit_description** | Option<**String**> | Description of the update (as appear in document history) |  |
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

