# \PermissionsApi

All URIs are relative to *https://onyourleft.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_permissions**](PermissionsApi.md#get_all_permissions) | **get** /rest/api/3/permissions | Get all permissions
[**get_bulk_permissions_post**](PermissionsApi.md#get_bulk_permissions_post) | **post** /rest/api/3/permissions/check | Get bulk permissions
[**get_my_permissions**](PermissionsApi.md#get_my_permissions) | **get** /rest/api/3/mypermissions | Get my permissions
[**get_permitted_projects_post**](PermissionsApi.md#get_permitted_projects_post) | **post** /rest/api/3/permissions/project | Get permitted projects



## get_all_permissions

> crate::models::Permissions get_all_permissions()
Get all permissions

Returns all permissions, including:   *  global permissions.  *  project permissions.  *  global permissions added by plugins.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Permissions**](Permissions.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bulk_permissions_post

> crate::models::BulkPermissionGrants get_bulk_permissions_post(bulk_permissions_request_bean)
Get bulk permissions

Returns:   *  for a list of global permissions, the global permissions granted to the user.  *  for a list of project permissions and lists of projects and issues, for each project permission a list of the projects and issues the user can access or manipulate.  Note that:   *  Invalid project and issue IDs are ignored.  *  A maximum of 1000 projects and 1000 issues can be checked.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bulk_permissions_request_bean** | [**BulkPermissionsRequestBean**](BulkPermissionsRequestBean.md) | Details of the permissions to check. | [required] |

### Return type

[**crate::models::BulkPermissionGrants**](BulkPermissionGrants.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_my_permissions

> crate::models::Permissions get_my_permissions(project_key, project_id, issue_key, issue_id, permissions, project_uuid, project_configuration_uuid)
Get my permissions

Returns a list of permissions indicating which permissions the user has. Details of the user's permissions can be obtained in a global, project, or issue context.  The user is reported as having a project permission:   *  in the global context, if the user has the project permission in any project.  *  for a project, where the project permission is determined using issue data, if the user meets the permission's criteria for any issue in the project. Otherwise, if the user has the project permission in the project.  *  for an issue, where a project permission is determined using issue data, if the user has the permission in the issue. Otherwise, if the user has the project permission in the project containing the issue.  This means that users may be shown as having an issue permission (such as EDIT\\_ISSUE) in the global context or a project context but may not have the permission for any or all issues. For example, if Reporters have the EDIT\\_ISSUE permission a user would be shown as having this permission in the global context or the context of a project, because any user can be a reporter. However, if they are not the user who reported the issue queried they would not have EDIT\\_ISSUE permission for that issue.  Global permissions are unaffected by context.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**project_key** | Option<**String**> | The key of project. Ignored if `projectId` is provided. |  |
**project_id** | Option<**String**> | The ID of project. |  |
**issue_key** | Option<**String**> | The key of the issue. Ignored if `issueId` is provided. |  |
**issue_id** | Option<**String**> | The ID of the issue. |  |
**permissions** | Option<**String**> | A list of permission keys. This parameter accepts a comma-separated list. [ Omitting this parameter is **deprecated**.](https://developer.atlassian.com/cloud/jira/platform/change-notice-get-my-permissions-requires-permissions-query-parameter/) To get the list of available permissions, use [Get all permissions](#api-rest-api-3-permissions-get). Note that deprecated keys cannot be used. Deprecated keys are not returned by [Get all permissions](#api-rest-api-3-permissions-get) but are returned by this operation if `permissions` is omitted. |  |
**project_uuid** | Option<**String**> |  |  |
**project_configuration_uuid** | Option<**String**> |  |  |

### Return type

[**crate::models::Permissions**](Permissions.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permitted_projects_post

> crate::models::PermittedProjects get_permitted_projects_post(permissions_keys_bean)
Get permitted projects

Returns all the projects where the user is granted a list of project permissions.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** None.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**permissions_keys_bean** | [**PermissionsKeysBean**](PermissionsKeysBean.md) |  | [required] |

### Return type

[**crate::models::PermittedProjects**](PermittedProjects.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

