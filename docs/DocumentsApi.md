# \DocumentsApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**copy_workspace**](DocumentsApi.md#copy_workspace) | **Post** /api/documents/{did}/workspaces/{wid}/copy | 
[**create_document**](DocumentsApi.md#create_document) | **Post** /api/documents | Create document.
[**create_version**](DocumentsApi.md#create_version) | **Post** /api/documents/d/{did}/versions | 
[**create_workspace**](DocumentsApi.md#create_workspace) | **Post** /api/documents/d/{did}/workspaces | 
[**delete7**](DocumentsApi.md#delete7) | **Delete** /api/documents/{did} | 
[**delete_workspace**](DocumentsApi.md#delete_workspace) | **Delete** /api/documents/d/{did}/workspaces/{wid} | 
[**download_external_data**](DocumentsApi.md#download_external_data) | **Get** /api/documents/d/{did}/externaldata/{fid} | Download External Data
[**export2_json**](DocumentsApi.md#export2_json) | **Post** /api/documents/d/{did}/{wv}/{wvid}/e/{eid}/export | 
[**get_acl**](DocumentsApi.md#get_acl) | **Get** /api/documents/{did}/acl | 
[**get_current_microversion**](DocumentsApi.md#get_current_microversion) | **Get** /api/documents/d/{did}/{wv}/{wvid}/currentmicroversion | Get Current Document Microversion
[**get_document**](DocumentsApi.md#get_document) | **Get** /api/documents/{did} | Get Document
[**get_document_permission_set**](DocumentsApi.md#get_document_permission_set) | **Get** /api/documents/{did}/permissionset | 
[**get_documents**](DocumentsApi.md#get_documents) | **Get** /api/documents | Get Documents
[**get_elements1**](DocumentsApi.md#get_elements1) | **Get** /api/documents/d/{did}/{wvm}/{wvmid}/elements | Get a list of elements in the workspace, version, or microversion of the document.
[**get_insertables**](DocumentsApi.md#get_insertables) | **Get** /api/documents/d/{did}/{wvm}/{wvmid}/insertables | 
[**get_version**](DocumentsApi.md#get_version) | **Get** /api/documents/d/{did}/versions/{vid} | Get Version
[**get_versions1**](DocumentsApi.md#get_versions1) | **Get** /api/documents/d/{did}/versions | Get Versions
[**get_workspaces1**](DocumentsApi.md#get_workspaces1) | **Get** /api/documents/d/{did}/workspaces | Get Workspaces
[**merge_into_workspace**](DocumentsApi.md#merge_into_workspace) | **Post** /api/documents/{did}/workspaces/{wid}/merge | Merge into workspace
[**restore_rendition**](DocumentsApi.md#restore_rendition) | **Post** /api/documents/{did}/workspaces/{wid}/restore/{mvid} | 
[**sync_application_elements**](DocumentsApi.md#sync_application_elements) | **Post** /api/documents/d/{did}/w/{wid}/syncApplicationElements | 
[**update_external_references_to_latest_documents**](DocumentsApi.md#update_external_references_to_latest_documents) | **Post** /api/documents/d/{did}/w/{wid}/e/{eid}/latestdocumentreferences | 



## copy_workspace

> copy_workspace(did, wid, bt_copy_document_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_copy_document_params** | Option<[**BtCopyDocumentParams**](BtCopyDocumentParams.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_document

> crate::models::BtDocumentInfo create_document(bt_document_params)
Create document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bt_document_params** | [**BtDocumentParams**](BtDocumentParams.md) |  | [required] |

### Return type

[**crate::models::BtDocumentInfo**](BTDocumentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_version

> create_version(did, bt_version_or_workspace_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**bt_version_or_workspace_params** | [**BtVersionOrWorkspaceParams**](BtVersionOrWorkspaceParams.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_workspace

> create_workspace(did, bt_version_or_workspace_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**bt_version_or_workspace_params** | Option<[**BtVersionOrWorkspaceParams**](BtVersionOrWorkspaceParams.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete7

> delete7(did, forever)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**forever** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workspace

> delete_workspace(did, wid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_external_data

> std::path::PathBuf download_external_data(did, fid, if_none_match)
Download External Data

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**fid** | **String** |  | [required] |
**if_none_match** | Option<**String**> |  |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09, application/vnd.onshape.v1+octet-stream;charset=UTF-8;qs=0.1, application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export2_json

> export2_json(did, wv, wvid, eid, bt_export_model_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_export_model_params** | Option<[**BtExportModelParams**](BtExportModelParams.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+octet-stream;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_acl

> get_acl(did)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_current_microversion

> crate::models::BtMicroversionInfo get_current_microversion(did, wv, wvid)
Get Current Document Microversion

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |

### Return type

[**crate::models::BtMicroversionInfo**](BTMicroversionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document

> crate::models::BtDocumentInfo get_document(did)
Get Document

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |

### Return type

[**crate::models::BtDocumentInfo**](BTDocumentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document_permission_set

> get_document_permission_set(did)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_documents

> crate::models::BtGlobalTreeNodeListResponse get_documents(q, filter, owner, owner_type, sort_column, sort_order, offset, limit, label, project, parent_id)
Get Documents

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**q** | Option<**String**> |  |  |[default to ]
**filter** | Option<**i32**> |  |  |
**owner** | Option<**String**> |  |  |[default to ]
**owner_type** | Option<**i32**> |  |  |[default to 1]
**sort_column** | Option<**String**> |  |  |[default to createdAt]
**sort_order** | Option<**String**> |  |  |[default to desc]
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 20]
**label** | Option<**String**> |  |  |
**project** | Option<**String**> |  |  |
**parent_id** | Option<**String**> |  |  |

### Return type

[**crate::models::BtGlobalTreeNodeListResponse**](BTGlobalTreeNodeListResponse.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_elements1

> Vec<crate::models::BtDocumentElementInfo> get_elements1(did, wvm, wvmid, element_type, element_id, with_thumbnails, link_document_id)
Get a list of elements in the workspace, version, or microversion of the document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**element_type** | Option<**String**> |  |  |[default to ]
**element_id** | Option<**String**> |  |  |[default to ]
**with_thumbnails** | Option<**bool**> |  |  |[default to false]
**link_document_id** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::BtDocumentElementInfo>**](BTDocumentElementInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_insertables

> get_insertables(did, wvm, wvmid, beta_capability_ids, include_parts, include_surfaces, include_wires, include_sketches, include_reference_features, include_assemblies, include_features, include_feature_studios, include_part_studios, include_blobs, include_meshes, include_flattened_bodies, allowed_blob_mime_types, max_feature_script_version, include_applications, allowed_application_mime_types)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**beta_capability_ids** | Option<[**Vec<String>**](String.md)> |  |  |
**include_parts** | Option<**bool**> |  |  |[default to false]
**include_surfaces** | Option<**bool**> |  |  |[default to false]
**include_wires** | Option<**bool**> |  |  |[default to false]
**include_sketches** | Option<**bool**> |  |  |[default to false]
**include_reference_features** | Option<**bool**> |  |  |[default to false]
**include_assemblies** | Option<**bool**> |  |  |[default to false]
**include_features** | Option<**bool**> |  |  |[default to false]
**include_feature_studios** | Option<**bool**> |  |  |[default to false]
**include_part_studios** | Option<**bool**> |  |  |[default to false]
**include_blobs** | Option<**bool**> |  |  |[default to false]
**include_meshes** | Option<**bool**> |  |  |[default to false]
**include_flattened_bodies** | Option<**bool**> |  |  |[default to false]
**allowed_blob_mime_types** | Option<**String**> |  |  |[default to ]
**max_feature_script_version** | Option<**i32**> |  |  |[default to 0]
**include_applications** | Option<**bool**> |  |  |[default to false]
**allowed_application_mime_types** | Option<**String**> |  |  |[default to ]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_version

> crate::models::BtVersionInfo get_version(did, vid, parents, link_document_id)
Get Version

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**vid** | **String** |  | [required] |
**parents** | Option<**bool**> |  |  |[default to false]
**link_document_id** | Option<**String**> |  |  |

### Return type

[**crate::models::BtVersionInfo**](BTVersionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_versions1

> Vec<crate::models::BtVersionInfo> get_versions1(did, offset, limit)
Get Versions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 0]

### Return type

[**Vec<crate::models::BtVersionInfo>**](BTVersionInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workspaces1

> Vec<crate::models::BtWorkspaceInfo> get_workspaces1(did)
Get Workspaces

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |

### Return type

[**Vec<crate::models::BtWorkspaceInfo>**](BTWorkspaceInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## merge_into_workspace

> crate::models::BtDocumentMergeInfo merge_into_workspace(did, wid, bt_version_or_workspace_info)
Merge into workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_version_or_workspace_info** | [**BtVersionOrWorkspaceInfo**](BtVersionOrWorkspaceInfo.md) |  | [required] |

### Return type

[**crate::models::BtDocumentMergeInfo**](BTDocumentMergeInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restore_rendition

> restore_rendition(did, wid, mvid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**mvid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_application_elements

> sync_application_elements(did, wid, application_element_ids, description)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**application_element_ids** | [**Vec<String>**](String.md) |  | [required] |
**description** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_external_references_to_latest_documents

> update_external_references_to_latest_documents(did, wid, eid, bt_link_to_latest_document_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_link_to_latest_document_params** | Option<[**BtLinkToLatestDocumentParams**](BtLinkToLatestDocumentParams.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

