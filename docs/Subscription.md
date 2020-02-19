# Subscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**object** | Option<**String**> |  | [optional]
**application_fee_percent** | Option<**f64**> |  | [optional]
**billing** | Option<**String**> |  | [optional]
**cancel_at_period_end** | Option<**bool**> |  | [optional]
**canceled_at** | Option<**i64**> |  | [optional]
**created** | Option<**i64**> |  | [optional]
**current_period_end** | Option<**i64**> |  | [optional]
**current_period_start** | Option<**i64**> |  | [optional]
**customer** | Option<**String**> |  | [optional]
**days_until_due** | Option<**i32**> |  | [optional]
**discount** | Option<[**crate::models::Discount**](Discount.md)> |  | [optional]
**ended_at** | Option<**i64**> |  | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**plan** | Option<[**crate::models::Plan**](Plan.md)> |  | [optional]
**quantity** | Option<**i32**> |  | [optional]
**start** | Option<**i64**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**tax_percent** | Option<**f64**> |  | [optional]
**trial_end** | Option<**i64**> |  | [optional]
**trial_start** | Option<**i64**> |  | [optional]
**customer_object** | Option<[**crate::models::Customer**](Customer.md)> |  | [optional]
**subscription_items** | Option<[**crate::models::SubscriptionItemCollection**](SubscriptionItemCollection.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


