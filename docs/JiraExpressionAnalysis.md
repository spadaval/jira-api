# JiraExpressionAnalysis

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expression** | **String** | The analysed expression. | 
**errors** | Option<[**Vec<crate::models::JiraExpressionValidationError>**](JiraExpressionValidationError.md)> | A list of validation errors. Not included if the expression is valid. | [optional]
**valid** | **bool** | Whether the expression is valid and the interpreter will evaluate it. Note that the expression may fail at runtime (for example, if it executes too many expensive operations). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


