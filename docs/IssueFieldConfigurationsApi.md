# \IssueFieldConfigurationsApi

All URIs are relative to *https://onyourleft.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_field_configurations**](IssueFieldConfigurationsApi.md#get_all_field_configurations) | **get** /rest/api/3/fieldconfiguration | Get all field configurations
[**get_field_configuration_items**](IssueFieldConfigurationsApi.md#get_field_configuration_items) | **get** /rest/api/3/fieldconfiguration/{id}/fields | Get field configuration items
[**get_field_configuration_scheme_mappings**](IssueFieldConfigurationsApi.md#get_field_configuration_scheme_mappings) | **get** /rest/api/3/fieldconfigurationscheme/mapping | Get field configuration issue type items



## get_all_field_configurations

> crate::models::PageBeanFieldConfiguration get_all_field_configurations(start_at, max_results, is_default)
Get all field configurations

Returns a paginated list of all field configurations.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**is_default** | Option<**bool**> | If *true* returns the default field configuration only. |  |[default to false]

### Return type

[**crate::models::PageBeanFieldConfiguration**](PageBeanFieldConfiguration.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_field_configuration_items

> crate::models::PageBeanFieldConfigurationItem get_field_configuration_items(id, start_at, max_results)
Get field configuration items

Returns a paginated list of all fields for a configuration.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i64** | The ID of the field configuration. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**crate::models::PageBeanFieldConfigurationItem**](PageBeanFieldConfigurationItem.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_field_configuration_scheme_mappings

> crate::models::PageBeanFieldConfigurationIssueTypeItem get_field_configuration_scheme_mappings(field_configuration_scheme_id, start_at, max_results)
Get field configuration issue type items

Returns a paginated list of field configuration issue type items.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_configuration_scheme_id** | [**Vec<i64>**](i64.md) | The list of field configuration scheme IDs. To include multiple field configuration schemes separate IDs with ampersand: `fieldConfigurationSchemeId=10000&fieldConfigurationSchemeId=10001`. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]

### Return type

[**crate::models::PageBeanFieldConfigurationIssueTypeItem**](PageBeanFieldConfigurationIssueTypeItem.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

