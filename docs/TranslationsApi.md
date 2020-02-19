# \TranslationsApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_translation**](TranslationsApi.md#create_translation) | **Post** /api/translations/d/{did}/w/{wid} | 
[**delete_translation**](TranslationsApi.md#delete_translation) | **Delete** /api/translations/{tid} | 
[**get_document_translations**](TranslationsApi.md#get_document_translations) | **Get** /api/translations/d/{did} | 
[**get_translation**](TranslationsApi.md#get_translation) | **Get** /api/translations/{tid} | 
[**get_translator_formats5**](TranslationsApi.md#get_translator_formats5) | **Get** /api/translations/translationformats | 



## create_translation

> create_translation(did, wid, content_disposition, entity, media_type, message_body_workers, parent, providers, body_parts)


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


## delete_translation

> delete_translation(tid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document_translations

> get_document_translations(did, offset, limit)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**did** | **String** |  | [required] |
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 20]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_translation

> get_translation(tid)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_translator_formats5

> get_translator_formats5()


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

