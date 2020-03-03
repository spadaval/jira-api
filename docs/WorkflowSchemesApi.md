# \WorkflowSchemesApi

All URIs are relative to *https://onyourleft.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_workflow_scheme_post**](WorkflowSchemesApi.md#create_workflow_scheme_post) | **post** /rest/api/3/workflowscheme | Create workflow scheme
[**delete_default_workflow**](WorkflowSchemesApi.md#delete_default_workflow) | **delete** /rest/api/3/workflowscheme/{id}/default | Delete default workflow
[**delete_workflow_mapping**](WorkflowSchemesApi.md#delete_workflow_mapping) | **delete** /rest/api/3/workflowscheme/{id}/workflow | Delete issue types for workflow in workflow scheme
[**delete_workflow_scheme**](WorkflowSchemesApi.md#delete_workflow_scheme) | **delete** /rest/api/3/workflowscheme/{id} | Delete workflow scheme
[**delete_workflow_scheme_issue_type**](WorkflowSchemesApi.md#delete_workflow_scheme_issue_type) | **delete** /rest/api/3/workflowscheme/{id}/issuetype/{issueType} | Delete workflow for issue type in workflow scheme
[**get_all_workflow_schemes**](WorkflowSchemesApi.md#get_all_workflow_schemes) | **get** /rest/api/3/workflowscheme | Get all workflow schemes
[**get_default_workflow**](WorkflowSchemesApi.md#get_default_workflow) | **get** /rest/api/3/workflowscheme/{id}/default | Get default workflow
[**get_workflow**](WorkflowSchemesApi.md#get_workflow) | **get** /rest/api/3/workflowscheme/{id}/workflow | Get issue types for workflows in workflow scheme
[**get_workflow_scheme**](WorkflowSchemesApi.md#get_workflow_scheme) | **get** /rest/api/3/workflowscheme/{id} | Get workflow scheme
[**get_workflow_scheme_issue_type**](WorkflowSchemesApi.md#get_workflow_scheme_issue_type) | **get** /rest/api/3/workflowscheme/{id}/issuetype/{issueType} | Get workflow for issue type in workflow scheme
[**set_workflow_scheme_issue_type_put**](WorkflowSchemesApi.md#set_workflow_scheme_issue_type_put) | **put** /rest/api/3/workflowscheme/{id}/issuetype/{issueType} | Set workflow for issue type in workflow scheme
[**update_default_workflow_put**](WorkflowSchemesApi.md#update_default_workflow_put) | **put** /rest/api/3/workflowscheme/{id}/default | Update default workflow
[**update_workflow_mapping_put**](WorkflowSchemesApi.md#update_workflow_mapping_put) | **put** /rest/api/3/workflowscheme/{id}/workflow | Set issue types for workflow in workflow scheme
[**update_workflow_scheme_put**](WorkflowSchemesApi.md#update_workflow_scheme_put) | **put** /rest/api/3/workflowscheme/{id} | Update workflow scheme



## create_workflow_scheme_post

> crate::models::WorkflowScheme create_workflow_scheme_post(workflow_scheme)
Create workflow scheme

Creates a workflow scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workflow_scheme** | [**WorkflowScheme**](WorkflowScheme.md) |  | [required] |

### Return type

[**crate::models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_default_workflow

> crate::models::WorkflowScheme delete_default_workflow(id, update_draft_if_needed)
Delete default workflow

Resets the default workflow for a workflow scheme. That is, the default workflow is set to Jira's system workflow (the *jira* workflow).  Note that active workflow schemes cannot be edited. If the workflow scheme is active, set `updateDraftIfNeeded` to `true` and a draft workflow scheme is created or updated with the default workflow reset. The draft workflow scheme can be published in Jira.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme. | [required] |
**update_draft_if_needed** | Option<**bool**> | Set to true to create or update the draft of a workflow scheme and delete the mapping from the draft, when the workflow scheme cannot be edited. Defaults to `false`. |  |

### Return type

[**crate::models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workflow_mapping

> delete_workflow_mapping(id, workflow_name, update_draft_if_needed)
Delete issue types for workflow in workflow scheme

Deletes the workflow-issue type mapping for a workflow in a workflow scheme.  Note that active workflow schemes cannot be edited. If the workflow scheme is active, set `updateDraftIfNeeded` to `true` and a draft workflow scheme is created or updated with the workflow-issue type mapping deleted. The draft workflow scheme can be published in Jira.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme. | [required] |
**workflow_name** | **String** | The name of the workflow. | [required] |
**update_draft_if_needed** | Option<**bool**> | Set to true to create or update the draft of a workflow scheme and delete the mapping from the draft, when the workflow scheme cannot be edited. Defaults to `false`. |  |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workflow_scheme

> delete_workflow_scheme(id)
Delete workflow scheme

Deletes a workflow scheme. Note that a workflow scheme cannot be deleted if it is active (that is, being used by at least one project).  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme. Find this ID by editing the desired workflow scheme in Jira. The ID is shown in the URL as `schemeId`. For example, *schemeId=10301*. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_workflow_scheme_issue_type

> crate::models::WorkflowScheme delete_workflow_scheme_issue_type(id, issue_type, update_draft_if_needed)
Delete workflow for issue type in workflow scheme

Deletes the issue type-workflow mapping for an issue type in a workflow scheme.  Note that active workflow schemes cannot be edited. If the workflow scheme is active, set `updateDraftIfNeeded` to `true` and a draft workflow scheme is created or updated with the issue type-workflow mapping deleted. The draft workflow scheme can be published in Jira.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme. | [required] |
**issue_type** | **String** | The ID of the issue type. | [required] |
**update_draft_if_needed** | Option<**bool**> | Set to true to create or update the draft of a workflow scheme and update the mapping in the draft, when the workflow scheme cannot be edited. Defaults to `false`. |  |

### Return type

[**crate::models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_workflow_schemes

> crate::models::PageBeanWorkflowScheme get_all_workflow_schemes(start_at, max_results)
Get all workflow schemes

Returns a paginated list of all workflow schemes, not including draft workflow schemes.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**crate::models::PageBeanWorkflowScheme**](PageBeanWorkflowScheme.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_workflow

> crate::models::DefaultWorkflow get_default_workflow(id, return_draft_if_exists)
Get default workflow

Returns the default workflow for a workflow scheme. The default workflow is the workflow that is assigned any issue types that have not been mapped to any other workflow. The default workflow has *All Unassigned Issue Types* listed in its issue types for the workflow scheme in Jira.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme. | [required] |
**return_draft_if_exists** | Option<**bool**> | Set to `true` to return the default workflow for the workflow scheme's draft rather than scheme itself. If the workflow scheme does not have a draft, then the default workflow for the workflow scheme is returned. |  |[default to false]

### Return type

[**crate::models::DefaultWorkflow**](DefaultWorkflow.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow

> crate::models::IssueTypesWorkflowMapping get_workflow(id, workflow_name, return_draft_if_exists)
Get issue types for workflows in workflow scheme

Returns the workflow-issue type mappings for a workflow scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme. | [required] |
**workflow_name** | Option<**String**> | The name of a workflow in the scheme. Limits the results to the workflow-issue type mapping for the specified workflow. |  |
**return_draft_if_exists** | Option<**bool**> | Returns the mapping from the workflow scheme's draft rather than the workflow scheme, if set to true. If no draft exists, the mapping from the workflow scheme is returned. |  |[default to false]

### Return type

[**crate::models::IssueTypesWorkflowMapping**](IssueTypesWorkflowMapping.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_scheme

> crate::models::WorkflowScheme get_workflow_scheme(id, return_draft_if_exists)
Get workflow scheme

Returns a workflow scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme. Find this ID by editing the desired workflow scheme in Jira. The ID is shown in the URL as `schemeId`. For example, *schemeId=10301*. | [required] |
**return_draft_if_exists** | Option<**bool**> | Returns the workflow scheme's draft rather than scheme itself, if set to true. If the workflow scheme does not have a draft, then the workflow scheme is returned. |  |[default to false]

### Return type

[**crate::models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_workflow_scheme_issue_type

> crate::models::IssueTypeWorkflowMapping get_workflow_scheme_issue_type(id, issue_type, return_draft_if_exists)
Get workflow for issue type in workflow scheme

Returns the issue type-workflow mapping for an issue type in a workflow scheme.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme. | [required] |
**issue_type** | **String** | The ID of the issue type. | [required] |
**return_draft_if_exists** | Option<**bool**> | Returns the mapping from the workflow scheme's draft rather than the workflow scheme, if set to true. If no draft exists, the mapping from the workflow scheme is returned. |  |[default to false]

### Return type

[**crate::models::IssueTypeWorkflowMapping**](IssueTypeWorkflowMapping.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_workflow_scheme_issue_type_put

> crate::models::WorkflowScheme set_workflow_scheme_issue_type_put(id, issue_type, issue_type_workflow_mapping)
Set workflow for issue type in workflow scheme

Sets the workflow for an issue type in a workflow scheme.  Note that active workflow schemes cannot be edited. If the workflow scheme is active, set `updateDraftIfNeeded` to `true` in the request body and a draft workflow scheme is created or updated with the new issue type-workflow mapping. The draft workflow scheme can be published in Jira.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme. | [required] |
**issue_type** | **String** | The ID of the issue type. | [required] |
**issue_type_workflow_mapping** | [**IssueTypeWorkflowMapping**](IssueTypeWorkflowMapping.md) | The issue type-project mapping. | [required] |

### Return type

[**crate::models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_default_workflow_put

> crate::models::WorkflowScheme update_default_workflow_put(id, default_workflow)
Update default workflow

Sets the default workflow for a workflow scheme.  Note that active workflow schemes cannot be edited. If the workflow scheme is active, set `updateDraftIfNeeded` to `true` in the request object and a draft workflow scheme is created or updated with the new default workflow. The draft workflow scheme can be published in Jira.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme. | [required] |
**default_workflow** | [**DefaultWorkflow**](DefaultWorkflow.md) | The new default workflow. | [required] |

### Return type

[**crate::models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workflow_mapping_put

> crate::models::WorkflowScheme update_workflow_mapping_put(id, workflow_name, issue_types_workflow_mapping)
Set issue types for workflow in workflow scheme

Sets the issue types for a workflow in a workflow scheme. The workflow can also be set as the default workflow for the workflow scheme. Unmapped issues types are mapped to the default workflow.  Note that active workflow schemes cannot be edited. If the workflow scheme is active, set `updateDraftIfNeeded` to `true` in the request body and a draft workflow scheme is created or updated with the new workflow-issue types mappings. The draft workflow scheme can be published in Jira.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme. | [required] |
**workflow_name** | **String** | The name of the workflow. | [required] |
**issue_types_workflow_mapping** | [**IssueTypesWorkflowMapping**](IssueTypesWorkflowMapping.md) |  | [required] |

### Return type

[**crate::models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_workflow_scheme_put

> crate::models::WorkflowScheme update_workflow_scheme_put(id, workflow_scheme)
Update workflow scheme

Updates a workflow scheme, including the name, default workflow, issue type to project mappings, and more. If the workflow scheme is active (that is, being used by at least one project), then a draft workflow scheme is created or updated instead, provided that `updateDraftIfNeeded` is set to `true`.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the workflow scheme. Find this ID by editing the desired workflow scheme in Jira. The ID is shown in the URL as `schemeId`. For example, *schemeId=10301*. | [required] |
**workflow_scheme** | [**WorkflowScheme**](WorkflowScheme.md) |  | [required] |

### Return type

[**crate::models::WorkflowScheme**](WorkflowScheme.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

