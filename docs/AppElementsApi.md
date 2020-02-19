# \AppElementsApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**commit_transaction**](AppElementsApi.md#commit_transaction) | **Post** /api/appelements/d/{did}/w/{wid}/e/{eid}/transactions/{tid} | Commit Transaction
[**create4**](AppElementsApi.md#create4) | **Post** /api/appelements/d/{did}/w/{wid} | Create Element.
[**create_reference**](AppElementsApi.md#create_reference) | **Post** /api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/references | Create Reference
[**delete_content**](AppElementsApi.md#delete_content) | **Delete** /api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/subelements/{sid} | Delete a Sub-element
[**delete_reference**](AppElementsApi.md#delete_reference) | **Delete** /api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/references/{rid} | Delete Reference
[**get_history**](AppElementsApi.md#get_history) | **Get** /api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/history | Get History
[**get_sub_element_content**](AppElementsApi.md#get_sub_element_content) | **Get** /api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content | Get Content
[**get_subelement_ids**](AppElementsApi.md#get_subelement_ids) | **Get** /api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/ids | Get Sub-element IDs
[**resolve_reference**](AppElementsApi.md#resolve_reference) | **Get** /api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/references/{rid} | Resolve Reference
[**resolve_references**](AppElementsApi.md#resolve_references) | **Get** /api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/resolvereferences | 
[**start_transaction**](AppElementsApi.md#start_transaction) | **Post** /api/appelements/d/{did}/w/{wid}/e/{eid}/transactions | Start Transaction
[**update4**](AppElementsApi.md#update4) | **Post** /api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content | Update Element
[**update_reference**](AppElementsApi.md#update_reference) | **Post** /api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/references/{rid} | Update Reference



## commit_transaction

> crate::models::BtAppElementModifyInfo commit_transaction(did, eid, wid, tid, bt_app_element_commit_transaction_params)
Commit Transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**tid** | **String** |  | [required] |
**bt_app_element_commit_transaction_params** | [**BtAppElementCommitTransactionParams**](BtAppElementCommitTransactionParams.md) |  | [required] |

### Return type

[**crate::models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create4

> crate::models::BtAppElementModifyInfo create4(did, wid, bt_app_element_params)
Create Element.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_app_element_params** | [**BtAppElementParams**](BtAppElementParams.md) |  | [required] |

### Return type

[**crate::models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_reference

> crate::models::BtAppElementReferenceInfo create_reference(did, eid, wvm, wvmid, bt_app_element_reference_params)
Create Reference

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**bt_app_element_reference_params** | [**BtAppElementReferenceParams**](BtAppElementReferenceParams.md) |  | [required] |

### Return type

[**crate::models::BtAppElementReferenceInfo**](BTAppElementReferenceInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_content

> crate::models::BtAppElementModifyInfo delete_content(did, eid, wvm, wvmid, sid, transaction_id, parent_change_id, description)
Delete a Sub-element

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**sid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**parent_change_id** | Option<**String**> |  |  |
**description** | Option<**String**> |  |  |

### Return type

[**crate::models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_reference

> crate::models::BtAppElementReferenceInfo delete_reference(did, eid, wvm, wvmid, rid, transaction_id, parent_change_id, description)
Delete Reference

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**rid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**parent_change_id** | Option<**String**> |  |  |
**description** | Option<**String**> |  |  |

### Return type

[**crate::models::BtAppElementReferenceInfo**](BTAppElementReferenceInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_history

> crate::models::BtAppElementHistoryInfo get_history(did, eid, wvm, wvmid)
Get History

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |

### Return type

[**crate::models::BtAppElementHistoryInfo**](BTAppElementHistoryInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sub_element_content

> crate::models::BtAppElementContentInfo get_sub_element_content(did, eid, wvm, wvmid, transaction_id, change_id, base_change_id, subelement_id, link_document_id)
Get Content

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**change_id** | Option<**String**> |  |  |
**base_change_id** | Option<**String**> |  |  |
**subelement_id** | Option<**String**> |  |  |
**link_document_id** | Option<**String**> |  |  |

### Return type

[**crate::models::BtAppElementContentInfo**](BTAppElementContentInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subelement_ids

> crate::models::BtAppElementModifyInfo get_subelement_ids(did, eid, wvm, wvmid, transaction_id, change_id)
Get Sub-element IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**change_id** | Option<**String**> |  |  |

### Return type

[**crate::models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resolve_reference

> crate::models::BtAppElementReferenceResolveInfo resolve_reference(did, eid, wvm, wvmid, rid, transaction_id, parent_change_id, include_internal, link_document_id)
Resolve Reference

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**rid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**parent_change_id** | Option<**String**> |  |  |
**include_internal** | Option<**bool**> |  |  |[default to false]
**link_document_id** | Option<**String**> |  |  |

### Return type

[**crate::models::BtAppElementReferenceResolveInfo**](BTAppElementReferenceResolveInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resolve_references

> resolve_references(did, eid, wvm, wvmid, transaction_id, parent_change_id, include_internal, link_document_id, reference_ids)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**transaction_id** | Option<**String**> |  |  |
**parent_change_id** | Option<**String**> |  |  |
**include_internal** | Option<**bool**> |  |  |[default to false]
**link_document_id** | Option<**String**> |  |  |
**reference_ids** | Option<**String**> |  |  |[default to ]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_transaction

> crate::models::BtAppElementModifyInfo start_transaction(did, eid, wid, bt_app_element_start_transaction_params)
Start Transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_app_element_start_transaction_params** | [**BtAppElementStartTransactionParams**](BtAppElementStartTransactionParams.md) |  | [required] |

### Return type

[**crate::models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update4

> crate::models::BtAppElementModifyInfo update4(did, eid, wvm, wvmid, bt_app_element_update_params)
Update Element

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**bt_app_element_update_params** | [**BtAppElementUpdateParams**](BtAppElementUpdateParams.md) |  | [required] |

### Return type

[**crate::models::BtAppElementModifyInfo**](BTAppElementModifyInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_reference

> crate::models::BtAppElementReferenceInfo update_reference(did, eid, wvm, wvmid, rid, bt_app_element_reference_params)
Update Reference

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**rid** | **String** |  | [required] |
**bt_app_element_reference_params** | [**BtAppElementReferenceParams**](BtAppElementReferenceParams.md) |  | [required] |

### Return type

[**crate::models::BtAppElementReferenceInfo**](BTAppElementReferenceInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

