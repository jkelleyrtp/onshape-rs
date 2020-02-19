# \MetadataApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_veop_standard_content_metadata**](MetadataApi.md#get_veop_standard_content_metadata) | **Get** /api/metadata/standardcontent/d/{did}/v/{vid}/e/{eid}/{otype}/{oid}/p/{pid} | 
[**get_wmve_ps_metadata**](MetadataApi.md#get_wmve_ps_metadata) | **Get** /api/metadata/d/{did}/{wvm}/{wvmid}/e/{eid}/p | 
[**get_wmvep_metadata**](MetadataApi.md#get_wmvep_metadata) | **Get** /api/metadata/d/{did}/{wvm}/{wvmid}/e/{eid}/p/{pid} | 
[**get_wv_es_metadata**](MetadataApi.md#get_wv_es_metadata) | **Get** /api/metadata/d/{did}/{wv}/{wvid}/e | 
[**get_wv_metadata**](MetadataApi.md#get_wv_metadata) | **Get** /api/metadata/d/{did}/{wv}/{wvid} | 
[**get_wve_metadata**](MetadataApi.md#get_wve_metadata) | **Get** /api/metadata/d/{did}/{wv}/{wvid}/e/{eid} | 
[**update_veop_standard_content_part_metadata**](MetadataApi.md#update_veop_standard_content_part_metadata) | **Post** /api/metadata/standardcontent/d/{did}/v/{vid}/e/{eid}/{otype}/{oid}/p/{pid} | 
[**update_wv_metadata**](MetadataApi.md#update_wv_metadata) | **Post** /api/metadata/d/{did}/{wv}/{wvid} | 
[**update_wve_metadata**](MetadataApi.md#update_wve_metadata) | **Post** /api/metadata/d/{did}/{wv}/{wvid}/e/{eid} | 
[**update_wvep_metadata**](MetadataApi.md#update_wvep_metadata) | **Post** /api/metadata/d/{did}/{wvm}/{wvmid}/e/{eid}/p/{pid} | 



## get_veop_standard_content_metadata

> get_veop_standard_content_metadata(did, vid, eid, otype, oid, pid, configuration, link_document_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**vid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**otype** | **String** |  | [required] |
**oid** | **String** |  | [required] |
**pid** | **String** |  | [required] |
**configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wmve_ps_metadata

> get_wmve_ps_metadata(did, wvm, wvmid, eid, configuration, link_document_id, infer_metadata_owner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |
**infer_metadata_owner** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wmvep_metadata

> get_wmvep_metadata(did, wvm, wvmid, eid, pid, configuration, link_document_id, infer_metadata_owner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**pid** | **String** |  | [required] |
**configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |
**infer_metadata_owner** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wv_es_metadata

> get_wv_es_metadata(did, wv, wvid, link_document_id, infer_metadata_owner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |
**infer_metadata_owner** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wv_metadata

> get_wv_metadata(did, wv, wvid, link_document_id, infer_metadata_owner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |
**infer_metadata_owner** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wve_metadata

> get_wve_metadata(did, wv, wvid, eid, configuration, link_document_id, infer_metadata_owner)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**configuration** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |
**infer_metadata_owner** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_veop_standard_content_part_metadata

> update_veop_standard_content_part_metadata(did, vid, eid, otype, oid, pid, body, link_document_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**vid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**otype** | **String** |  | [required] |
**oid** | **String** |  | [required] |
**pid** | **String** |  | [required] |
**body** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_wv_metadata

> update_wv_metadata(did, wv, wvid, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_wve_metadata

> update_wve_metadata(did, wv, wvid, eid, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_wvep_metadata

> update_wvep_metadata(did, wvm, wvmid, eid, pid, sub_resource, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**pid** | **String** |  | [required] |
**sub_resource** | **String** |  | [required] |
**body** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

