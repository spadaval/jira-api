# \UserPropertiesApi

All URIs are relative to *https://onyourleft.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user_property**](UserPropertiesApi.md#delete_user_property) | **delete** /rest/api/3/user/properties/{propertyKey} | Delete user property
[**get_user_property**](UserPropertiesApi.md#get_user_property) | **get** /rest/api/3/user/properties/{propertyKey} | Get user property
[**get_user_property_keys**](UserPropertiesApi.md#get_user_property_keys) | **get** /rest/api/3/user/properties | Get user property keys
[**set_user_property_put**](UserPropertiesApi.md#set_user_property_put) | **put** /rest/api/3/user/properties/{propertyKey} | Set user property



## delete_user_property

> delete_user_property(property_key, account_id, user_key, username)
Delete user property

Deletes a property from a user.  Note: This operation does not access the [user properties](https://confluence.atlassian.com/x/8YxjL) created and maintained in Jira.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to delete a property from any user.  *  Access to Jira, to delete a property from the calling user's record.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property_key** | **String** | The key of the user's property. | [required] |
**account_id** | Option<**String**> | The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*. |  |
**user_key** | Option<**String**> | This parameter is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. |  |
**username** | Option<**String**> | This parameter is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. |  |

### Return type

 (empty response body)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_property

> crate::models::EntityProperty get_user_property(property_key, account_id, user_key, username)
Get user property

Returns the value of a user's property. If no property key is provided [Get user property keys](#api-rest-api-3-user-properties-get) is called.  Note: This operation does not access the [user properties](https://confluence.atlassian.com/x/8YxjL) created and maintained in Jira.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to get a property from any user.  *  Access to Jira, to get a property from the calling user's record.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property_key** | **String** | The key of the user's property. | [required] |
**account_id** | Option<**String**> | The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*. |  |
**user_key** | Option<**String**> | This parameter is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. |  |
**username** | Option<**String**> | This parameter is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. |  |

### Return type

[**crate::models::EntityProperty**](EntityProperty.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_property_keys

> crate::models::PropertyKeys get_user_property_keys(account_id, user_key, username)
Get user property keys

Returns the keys of all properties for a user.  Note: This operation does not access the [user properties](https://confluence.atlassian.com/x/8YxjL) created and maintained in Jira.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to access the property keys on any user.  *  Access to Jira, to access the calling user's property keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*. |  |
**user_key** | Option<**String**> | This parameter is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. |  |
**username** | Option<**String**> | This parameter is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. |  |

### Return type

[**crate::models::PropertyKeys**](PropertyKeys.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_property_put

> serde_json::Value set_user_property_put(property_key, body, account_id, user_key, username)
Set user property

Sets the value of a user's property. Use this resource to store custom data against a user.  Note: This operation does not access the [user properties](https://confluence.atlassian.com/x/8YxjL) created and maintained in Jira.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg), to set a property on any user.  *  Access to Jira, to set a property on the calling user's record.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property_key** | **String** | The key of the user's property. The maximum length is 255 characters. | [required] |
**body** | **serde_json::Value** |  | [required] |
**account_id** | Option<**String**> | The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*. |  |
**user_key** | Option<**String**> | This parameter is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. |  |
**username** | Option<**String**> | This parameter is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

