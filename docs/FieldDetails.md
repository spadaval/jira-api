# FieldDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the field. | [optional]
**key** | Option<**String**> | The key of the field. | [optional]
**name** | Option<**String**> | The name of the field. | [optional]
**custom** | Option<**bool**> | Whether the field is a custom field. | [optional]
**orderable** | Option<**bool**> | Whether the content of the field can be used to order lists. | [optional]
**navigable** | Option<**bool**> | Whether the field can be used as a column on the issue navigator. | [optional]
**searchable** | Option<**bool**> | Whether the content of the field can be searched. | [optional]
**clause_names** | Option<**Vec<String>**> | The names that can be used to reference the field in an advanced search. For more information, see [Advanced searching - fields reference](https://confluence.atlassian.com/x/gwORLQ). | [optional]
**scope** | Option<[**crate::models::Scope**](Scope.md)> | The scope of the field. | [optional]
**schema** | Option<[**crate::models::JsonTypeBean**](JsonTypeBean.md)> | The data schema for the field. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


