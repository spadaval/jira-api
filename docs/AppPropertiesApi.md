# \AppPropertiesApi

All URIs are relative to *https://onyourleft.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_addon_property**](AppPropertiesApi.md#delete_addon_property) | **delete** /rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey} | Delete app property
[**get_addon_properties**](AppPropertiesApi.md#get_addon_properties) | **get** /rest/atlassian-connect/1/addons/{addonKey}/properties | Get app properties
[**get_addon_property**](AppPropertiesApi.md#get_addon_property) | **get** /rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey} | Get app property
[**put_addon_property**](AppPropertiesApi.md#put_addon_property) | **put** /rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey} | Set app property



## delete_addon_property

> delete_addon_property(addon_key, property_key)
Delete app property

Deletes an app's property.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_key** | **String** | The key of the app, as defined in its descriptor. | [required] |
**property_key** | **String** | The key of the property. | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addon_properties

> crate::models::PropertyKeys get_addon_properties(addon_key)
Get app properties

Gets all the properties of an app.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_key** | **String** | The key of the app, as defined in its descriptor. | [required] |

### Return type

[**crate::models::PropertyKeys**](PropertyKeys.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_addon_property

> crate::models::EntityProperty get_addon_property(addon_key, property_key)
Get app property

Returns the key and value of an app's property.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_key** | **String** | The key of the app, as defined in its descriptor. | [required] |
**property_key** | **String** | The key of the property. | [required] |

### Return type

[**crate::models::EntityProperty**](EntityProperty.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_addon_property

> crate::models::OperationMessage put_addon_property(addon_key, property_key, body)
Set app property

Sets the value of an app's property. Use this resource to store custom data for your app.  The value of the request body must be a [valid](http://tools.ietf.org/html/rfc4627), non-empty JSON blob. The maximum length is 32768 characters.  **[Permissions](#permissions) required:** Only a Connect app whose key matches `addonKey` can make this request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**addon_key** | **String** | The key of the app, as defined in its descriptor. | [required] |
**property_key** | **String** | The key of the property. | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**crate::models::OperationMessage**](OperationMessage.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

