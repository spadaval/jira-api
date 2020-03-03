# JiraExpressionsComplexityBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**steps** | Option<[**crate::models::JiraExpressionsComplexityValueBean**](JiraExpressionsComplexityValueBean.md)> | The number of steps it took to evaluate the expression. | [optional]
**expensive_operations** | Option<[**crate::models::JiraExpressionsComplexityValueBean**](JiraExpressionsComplexityValueBean.md)> | The number of expensive operations executed while evaluating the expression. Expensive operations are those that load additional data, such as entity properties, comments, or custom fields. | [optional]
**beans** | Option<[**crate::models::JiraExpressionsComplexityValueBean**](JiraExpressionsComplexityValueBean.md)> | The number of Jira REST API beans returned in the response. | [optional]
**primitive_values** | Option<[**crate::models::JiraExpressionsComplexityValueBean**](JiraExpressionsComplexityValueBean.md)> | The number of primitive values returned in the response. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


