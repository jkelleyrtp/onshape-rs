# \ElementsApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**copy_element_from_source_document**](ElementsApi.md#copy_element_from_source_document) | **Post** /api/elements/copyelement/{did}/workspace/{wid} | 
[**decode_configuration**](ElementsApi.md#decode_configuration) | **Get** /api/elements/d/{did}/{wvm}/{wvmid}/e/{eid}/configurationencodings/{cid} | 
[**delete8**](ElementsApi.md#delete8) | **Delete** /api/elements/d/{did}/w/{wid}/e/{eid} | 
[**encode_configuration_map**](ElementsApi.md#encode_configuration_map) | **Post** /api/elements/d/{did}/e/{eid}/configurationencodings | 
[**get_configuration**](ElementsApi.md#get_configuration) | **Get** /api/elements/d/{did}/{wvm}/{wvmid}/e/{eid}/configuration | 
[**get_element_by_version_deprecated**](ElementsApi.md#get_element_by_version_deprecated) | **Get** /api/elements/{did}/version/{vid} | 
[**get_element_by_workspace_deprecated**](ElementsApi.md#get_element_by_workspace_deprecated) | **Get** /api/elements/{did}/workspace/{wid} | 
[**get_element_metadata**](ElementsApi.md#get_element_metadata) | **Get** /api/elements/d/{did}/{wv}/{wvid}/e/{eid}/metadata | 
[**get_element_metadata_deprecated**](ElementsApi.md#get_element_metadata_deprecated) | **Get** /api/elements/{emid} | 
[**get_element_translator_formats**](ElementsApi.md#get_element_translator_formats) | **Get** /api/elements/translatorFormats/{did}/{wid}/{eid} | 
[**get_translator_formats3**](ElementsApi.md#get_translator_formats3) | **Get** /api/elements/translatorFormats | 
[**update_configuration**](ElementsApi.md#update_configuration) | **Post** /api/elements/d/{did}/{wvm}/{wvmid}/e/{eid}/configuration | 
[**update_element_metadata**](ElementsApi.md#update_element_metadata) | **Post** /api/elements/d/{did}/{wv}/{wvid}/e/{eid}/metadata | 
[**update_references**](ElementsApi.md#update_references) | **Post** /api/elements/d/{did}/w/{wid}/e/{eid}/updatereferences | 
[**upload_file1**](ElementsApi.md#upload_file1) | **Post** /api/elements/upload/{did} | 



## copy_element_from_source_document

> copy_element_from_source_document(did, wid, bt_copy_element_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_copy_element_params** | [**BtCopyElementParams**](BtCopyElementParams.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decode_configuration

> decode_configuration(did, wvm, wvmid, eid, cid, link_document_id, include_display, configuration_is_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**cid** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |
**include_display** | Option<**bool**> |  |  |[default to false]
**configuration_is_id** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete8

> delete8(did, wid, eid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encode_configuration_map

> encode_configuration_map(did, eid, bt_configuration_params, version_id, link_document_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_configuration_params** | [**BtConfigurationParams**](BtConfigurationParams.md) |  | [required] |
**version_id** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configuration

> get_configuration(did, wvm, wvmid, eid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_element_by_version_deprecated

> get_element_by_version_deprecated(did, vid, with_thumbnails)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**vid** | **String** |  | [required] |
**with_thumbnails** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_element_by_workspace_deprecated

> get_element_by_workspace_deprecated(did, wid, with_thumbnails)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**with_thumbnails** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_element_metadata

> get_element_metadata(did, wv, wvid, eid, link_document_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**link_document_id** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_element_metadata_deprecated

> get_element_metadata_deprecated(emid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_element_translator_formats

> get_element_translator_formats(did, wid, eid, check_content, configuration)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**check_content** | Option<**bool**> |  |  |[default to true]
**configuration** | Option<**String**> |  |  |[default to ]

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_translator_formats3

> get_translator_formats3()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_configuration

> update_configuration(did, wvm, wvmid, eid, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**body** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_element_metadata

> update_element_metadata(did, wv, wvid, eid, btpdm_metadata_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**btpdm_metadata_params** | [**BtpdmMetadataParams**](BtpdmMetadataParams.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_references

> update_references(did, wid, eid, bt_update_reference_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_update_reference_params** | [**BtUpdateReferenceParams**](BtUpdateReferenceParams.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file1

> upload_file1(did, element_id, workspace_id, content_disposition, entity, media_type, message_body_workers, parent, providers, body_parts)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**element_id** | Option<**String**> |  |  |
**workspace_id** | Option<**String**> |  |  |
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

