# \IssueResolutionsApi

All URIs are relative to *https://onyourleft.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_resolution**](IssueResolutionsApi.md#get_resolution) | **get** /rest/api/3/resolution/{id} | Get resolution
[**get_resolutions**](IssueResolutionsApi.md#get_resolutions) | **get** /rest/api/3/resolution | Get resolutions



## get_resolution

> crate::models::Resolution get_resolution(id)
Get resolution

Returns an issue resolution value.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the issue resolution value. | [required] |

### Return type

[**crate::models::Resolution**](Resolution.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resolutions

> Vec<crate::models::Resolution> get_resolutions()
Get resolutions

Returns a list of all issue resolution values.  **[Permissions](#permissions) required:** Permission to access Jira.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Resolution>**](Resolution.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

