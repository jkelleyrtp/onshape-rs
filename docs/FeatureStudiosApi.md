# \FeatureStudiosApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_feature_studio**](FeatureStudiosApi.md#create_feature_studio) | **Post** /api/featurestudios/d/{did}/w/{wid} | 
[**get_feature_studio_contents**](FeatureStudiosApi.md#get_feature_studio_contents) | **Get** /api/featurestudios/d/{did}/{wvm}/{wvmid}/e/{eid} | Get Feature Studio Contents.
[**get_feature_studio_specs**](FeatureStudiosApi.md#get_feature_studio_specs) | **Get** /api/featurestudios/d/{did}/{wvm}/{wvmid}/e/{eid}/featurespecs | 
[**update_feature_studio_contents**](FeatureStudiosApi.md#update_feature_studio_contents) | **Post** /api/featurestudios/d/{did}/{wvm}/{wvmid}/e/{eid} | 



## create_feature_studio

> create_feature_studio(did, wid, bt_model_element_params)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**bt_model_element_params** | [**BtModelElementParams**](BtModelElementParams.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feature_studio_contents

> crate::models::BtFeatureStudioContents get_feature_studio_contents(did, wvm, wvmid, eid)
Get Feature Studio Contents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wvm** | **String** |  | [required] |
**wvmid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

[**crate::models::BtFeatureStudioContents**](BTFeatureStudioContents.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_feature_studio_specs

> get_feature_studio_specs(did, wvm, wvmid, eid)


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


## update_feature_studio_contents

> update_feature_studio_contents(did, wvm, wvmid, eid, body)


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

