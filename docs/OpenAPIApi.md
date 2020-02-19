# \OpenAPIApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_open_api**](OpenAPIApi.md#get_open_api) | **Get** /api/openapi | OpenAPI spec documentation for the Onshape REST API.



## get_open_api

> crate::models::OpenApi get_open_api(file_type, excluded_tags, included_tags, include_deprecated, documentation_status)
OpenAPI spec documentation for the Onshape REST API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_type** | Option<**String**> | The type of file to return. Defaults to JSON. |  |[default to JSON]
**excluded_tags** | Option<**String**> | If an operation contains an excluded tag, it is not returned from this endpoint. |  |
**included_tags** | Option<**String**> | Return only operations with tags included in includedTags. |  |
**include_deprecated** | Option<**bool**> | Include deprecated endpoints. |  |
**documentation_status** | Option<[**Vec<String>**](String.md)> | Only return endpoints that have the specified document status. Default is to return all the endpoints the user should have access to. |  |

### Return type

[**crate::models::OpenApi**](OpenAPI.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

