# \DrawingsApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_translator_formats2**](DrawingsApi.md#get_translator_formats2) | **Get** /api/drawings/d/{did}/w/{wid}/e/{eid}/translationformats | 
[**translate_format4**](DrawingsApi.md#translate_format4) | **Post** /api/drawings/d/{did}/{wv}/{wvid}/e/{eid}/translations | Create Drawing translation



## get_translator_formats2

> get_translator_formats2(did, wid, eid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wid** | **String** |  | [required] |
**eid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## translate_format4

> crate::models::BtTranslationRequestInfo translate_format4(did, wv, wvid, eid, bt_translate_format_params)
Create Drawing translation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**wv** | **String** |  | [required] |
**wvid** | **String** |  | [required] |
**eid** | **String** |  | [required] |
**bt_translate_format_params** | [**BtTranslateFormatParams**](BtTranslateFormatParams.md) |  | [required] |

### Return type

[**crate::models::BtTranslationRequestInfo**](BTTranslationRequestInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

