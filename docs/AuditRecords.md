# AuditRecords

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**offset** | Option<**i32**> | The number of audit items skipped before the first item in this list. | [optional][readonly]
**limit** | Option<**i32**> | The requested or default limit on the number of audit items to be returned. | [optional][readonly]
**total** | Option<**i64**> | The total number of audit items returned. | [optional][readonly]
**records** | Option<[**Vec<crate::models::AuditRecordBean>**](AuditRecordBean.md)> | The list of audit items. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


