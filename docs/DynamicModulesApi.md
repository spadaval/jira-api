# \DynamicModulesApi

All URIs are relative to *https://onyourleft.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_modules**](DynamicModulesApi.md#get_modules) | **get** /rest/atlassian-connect/1/app/module/dynamic | Get modules
[**register_modules_post**](DynamicModulesApi.md#register_modules_post) | **post** /rest/atlassian-connect/1/app/module/dynamic | Register modules
[**remove_modules_delete**](DynamicModulesApi.md#remove_modules_delete) | **delete** /rest/atlassian-connect/1/app/module/dynamic | Remove modules



## get_modules

> ::std::collections::HashMap<String, Vec<serde_json::Value>> get_modules()
Get modules

Returns all modules registered dynamically by the calling app.  **[Permissions](#permissions) required:** Only Connect apps can make this request.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, Vec<serde_json::Value>>**](array.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_modules_post

> register_modules_post(request_body)
Register modules

Registers a list of modules.  **[Permissions](#permissions) required:** Only Connect apps can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**::std::collections::HashMap<String, Vec<serde_json::Value>>**](array.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_modules_delete

> remove_modules_delete(module_key)
Remove modules

Remove all or a list of modules registered by the calling app.  **[Permissions](#permissions) required:** Only Connect apps can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**module_key** | Option<[**Vec<String>**](String.md)> | The key of the module to remove. To include multiple module keys, provide multiple copies of this parameter. For example, `moduleKey=dynamic-attachment-entity-property&moduleKey=dynamic-select-field`. Nonexistent keys are ignored. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

