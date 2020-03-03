# Transition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the transition. | 
**name** | **String** | The name of the transition. | 
**description** | **String** | The description of the transition. | 
**from** | **Vec<String>** | The statuses the transition can start from. | 
**to** | **String** | The status the transition goes to. | 
**_type** | **String** | The type of the transition. | 
**screen** | Option<[**crate::models::ScreenId**](ScreenID.md)> |  | [optional]
**rules** | Option<[**crate::models::WorkflowRules**](WorkflowRules.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


