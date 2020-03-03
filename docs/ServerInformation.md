# ServerInformation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**base_url** | Option<**String**> | The base URL of the Jira instance. | [optional]
**version** | Option<**String**> | The version of Jira. | [optional]
**version_numbers** | Option<**Vec<i32>**> | The major, minor, and revision version numbers of the Jira version. | [optional]
**deployment_type** | Option<**String**> | The type of server deployment. This is always returned as *Cloud*. | [optional]
**build_number** | Option<**i32**> | The build number of the Jira version. | [optional]
**build_date** | Option<**String**> | The timestamp when the Jira version was built. | [optional]
**server_time** | Option<**String**> | The time in Jira when this request was responded to. | [optional]
**scm_info** | Option<**String**> | The unique identifier of the Jira version. | [optional]
**server_title** | Option<**String**> | The name of the Jira instance. | [optional]
**health_checks** | Option<[**Vec<crate::models::HealthCheckResult>**](HealthCheckResult.md)> | Jira instance health check results. Deprecated and no longer returned. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


