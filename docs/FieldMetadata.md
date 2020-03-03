# FieldMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**required** | **bool** | Whether the field is required. | [readonly]
**schema** | Option<[**crate::models::JsonTypeBean**](JsonTypeBean.md)> | The data type of the field. | [optional][readonly]
**name** | **String** | The name of the field. | [readonly]
**key** | **String** | The key of the field. | [readonly]
**auto_complete_url** | Option<**String**> | The URL that can be used to automatically complete the field. | [optional][readonly]
**has_default_value** | Option<**bool**> | Whether the field has a default value. | [optional][readonly]
**operations** | **Vec<String>** | The list of operations that can be performed on the field. | [readonly]
**allowed_values** | Option<[**Vec<serde_json::Value>**](serde_json::Value.md)> | The list of values allowed in the field. | [optional][readonly]
**default_value** | Option<[**serde_json::Value**](serde_json::Value.md)> | The default value of the field. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


