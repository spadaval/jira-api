# \IssueSecuritySchemesApi

All URIs are relative to *https://onyourleft.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_issue_security_scheme**](IssueSecuritySchemesApi.md#get_issue_security_scheme) | **get** /rest/api/3/issuesecurityschemes/{id} | Get issue security scheme
[**get_issue_security_schemes**](IssueSecuritySchemesApi.md#get_issue_security_schemes) | **get** /rest/api/3/issuesecurityschemes | Get issue security schemes



## get_issue_security_scheme

> crate::models::SecurityScheme get_issue_security_scheme(id)
Get issue security scheme

Returns an issue security scheme along with its security levels.  This operation can be accessed anonymously.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *  *Administer Projects* [project permission](https://confluence.atlassian.com/x/yodKLg) for a project that uses the requested issue security scheme.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the issue security scheme. Use the [Get issue security schemes](#api-rest-api-3-issuesecurityschemes-get) operation to get a list of issue security scheme IDs. | [required] |

### Return type

[**crate::models::SecurityScheme**](SecurityScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_security_schemes

> crate::models::SecuritySchemes get_issue_security_schemes()
Get issue security schemes

Returns all [issue security schemes](https://confluence.atlassian.com/x/J4lKLg).  This operation can be accessed anonymously.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SecuritySchemes**](SecuritySchemes.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

