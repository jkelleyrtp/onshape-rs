# Customer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**object** | Option<**String**> |  | [optional]
**account_balance** | Option<**i64**> |  | [optional]
**business_vat_id** | Option<**String**> |  | [optional]
**created** | Option<**i64**> |  | [optional]
**currency** | Option<**String**> |  | [optional]
**default_source** | Option<**String**> |  | [optional]
**deleted** | Option<**bool**> |  | [optional]
**delinquent** | Option<**bool**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**discount** | Option<[**crate::models::Discount**](Discount.md)> |  | [optional]
**email** | Option<**String**> |  | [optional]
**livemode** | Option<**bool**> |  | [optional]
**metadata** | Option<**::std::collections::HashMap<String, String>**> |  | [optional]
**shipping** | Option<[**crate::models::ShippingDetails**](ShippingDetails.md)> |  | [optional]
**sources** | Option<[**crate::models::ExternalAccountCollection**](ExternalAccountCollection.md)> |  | [optional]
**subscriptions** | Option<[**crate::models::CustomerSubscriptionCollection**](CustomerSubscriptionCollection.md)> |  | [optional]
**cards** | Option<[**crate::models::CustomerCardCollection**](CustomerCardCollection.md)> |  | [optional]
**default_card** | Option<**String**> |  | [optional]
**next_recurring_charge** | Option<[**crate::models::NextRecurringCharge**](NextRecurringCharge.md)> |  | [optional]
**subscription** | Option<[**crate::models::Subscription**](Subscription.md)> |  | [optional]
**trial_end** | Option<**i64**> |  | [optional]
**default_source_object** | Option<[**crate::models::ExternalAccount**](ExternalAccount.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


