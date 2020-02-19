# \AccountsApi

All URIs are relative to *https://cad.onshape.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_purchase_new**](AccountsApi.md#cancel_purchase_new) | **Delete** /api/accounts/{aid}/purchases/{pid} | Cancel Recurring Subscription
[**consume_purchase**](AccountsApi.md#consume_purchase) | **Post** /api/accounts/purchases/{pid}/consume | Mark Purchase Consumed For User
[**get_plan_purchases**](AccountsApi.md#get_plan_purchases) | **Get** /api/accounts/plans/{planId}/purchases | Get Plan Purchases
[**get_purchases**](AccountsApi.md#get_purchases) | **Get** /api/accounts/purchases | Get User's Appstore Purchases.



## cancel_purchase_new

> cancel_purchase_new(aid, pid, cancel_immediately)
Cancel Recurring Subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**aid** | **String** |  | [required] |
**pid** | **String** |  | [required] |
**cancel_immediately** | Option<**bool**> |  |  |[default to false]

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## consume_purchase

> crate::models::BtPurchaseInfo consume_purchase(pid, bt_purchase_user_params)
Mark Purchase Consumed For User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pid** | **String** |  | [required] |
**bt_purchase_user_params** | Option<[**BtPurchaseUserParams**](BtPurchaseUserParams.md)> |  |  |

### Return type

[**crate::models::BtPurchaseInfo**](BTPurchaseInfo.md)

### Authorization

[OAuth2](../README.md#OAuth2)

### HTTP request headers

- **Content-Type**: application/json;charset=UTF-8; qs=0.09
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_plan_purchases

> crate::models::BtListResponseBtPurchaseInfo get_plan_purchases(plan_id, offset, limit)
Get Plan Purchases

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **String** |  | [required] |
**offset** | Option<**i32**> |  |  |[default to 0]
**limit** | Option<**i32**> |  |  |[default to 20]

### Return type

[**crate::models::BtListResponseBtPurchaseInfo**](BTListResponseBTPurchaseInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_purchases

> Vec<crate::models::BtPurchaseInfo> get_purchases(all, own_purchase_only)
Get User's Appstore Purchases.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**all** | Option<**bool**> |  |  |[default to false]
**own_purchase_only** | Option<**bool**> |  |  |[default to false]

### Return type

[**Vec<crate::models::BtPurchaseInfo>**](BTPurchaseInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.onshape.v1+json;charset=UTF-8;qs=0.1, application/json;charset=UTF-8; qs=0.09

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

