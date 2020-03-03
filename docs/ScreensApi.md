# \ScreensApi

All URIs are relative to *https://onyourleft.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_field_to_default_screen_post**](ScreensApi.md#add_field_to_default_screen_post) | **post** /rest/api/3/screens/addToDefault/{fieldId} | Add field to default screen
[**add_screen_tab_field_post**](ScreensApi.md#add_screen_tab_field_post) | **post** /rest/api/3/screens/{screenId}/tabs/{tabId}/fields | Add screen tab field
[**add_screen_tab_post**](ScreensApi.md#add_screen_tab_post) | **post** /rest/api/3/screens/{screenId}/tabs | Create screen tab
[**delete_screen_tab**](ScreensApi.md#delete_screen_tab) | **delete** /rest/api/3/screens/{screenId}/tabs/{tabId} | Delete screen tab
[**get_all_screen_schemes**](ScreensApi.md#get_all_screen_schemes) | **get** /rest/api/3/screenscheme | Get all screen schemes
[**get_all_screen_tab_fields**](ScreensApi.md#get_all_screen_tab_fields) | **get** /rest/api/3/screens/{screenId}/tabs/{tabId}/fields | Get all screen tab fields
[**get_all_screen_tabs**](ScreensApi.md#get_all_screen_tabs) | **get** /rest/api/3/screens/{screenId}/tabs | Get all screen tabs
[**get_all_screens**](ScreensApi.md#get_all_screens) | **get** /rest/api/3/screens | Get all screens
[**get_available_screen_fields**](ScreensApi.md#get_available_screen_fields) | **get** /rest/api/3/screens/{screenId}/availableFields | Get available screen fields
[**get_issue_type_screen_scheme_mappings**](ScreensApi.md#get_issue_type_screen_scheme_mappings) | **get** /rest/api/3/issuetypescreenscheme/mapping | Get issue type screen scheme items
[**get_issue_type_screen_scheme_project_associations**](ScreensApi.md#get_issue_type_screen_scheme_project_associations) | **get** /rest/api/3/issuetypescreenscheme/project | Get issue type screen schemes for projects
[**get_screens_for_field**](ScreensApi.md#get_screens_for_field) | **get** /rest/api/3/field/{fieldId}/screens | Get screens for a field
[**move_screen_tab_field_post**](ScreensApi.md#move_screen_tab_field_post) | **post** /rest/api/3/screens/{screenId}/tabs/{tabId}/fields/{id}/move | Move screen tab field
[**move_screen_tab_post**](ScreensApi.md#move_screen_tab_post) | **post** /rest/api/3/screens/{screenId}/tabs/{tabId}/move/{pos} | Move screen tab
[**remove_screen_tab_field_delete**](ScreensApi.md#remove_screen_tab_field_delete) | **delete** /rest/api/3/screens/{screenId}/tabs/{tabId}/fields/{id} | Remove screen tab field
[**rename_screen_tab_put**](ScreensApi.md#rename_screen_tab_put) | **put** /rest/api/3/screens/{screenId}/tabs/{tabId} | Update screen tab



## add_field_to_default_screen_post

> serde_json::Value add_field_to_default_screen_post(field_id)
Add field to default screen

Adds a field to the default tab of the default screen.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the field. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_screen_tab_field_post

> crate::models::ScreenableField add_screen_tab_field_post(screen_id, tab_id, add_field_bean)
Add screen tab field

Adds a field to a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |
**add_field_bean** | [**AddFieldBean**](AddFieldBean.md) |  | [required] |

### Return type

[**crate::models::ScreenableField**](ScreenableField.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_screen_tab_post

> crate::models::ScreenableTab add_screen_tab_post(screen_id, screenable_tab)
Create screen tab

Creates a tab for a screen.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**screenable_tab** | [**ScreenableTab**](ScreenableTab.md) |  | [required] |

### Return type

[**crate::models::ScreenableTab**](ScreenableTab.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_screen_tab

> delete_screen_tab(screen_id, tab_id)
Delete screen tab

Deletes a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_screen_schemes

> crate::models::PageBeanScreenScheme get_all_screen_schemes(start_at, max_results)
Get all screen schemes

Returns all screen schemes.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 25]

### Return type

[**crate::models::PageBeanScreenScheme**](PageBeanScreenScheme.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_screen_tab_fields

> Vec<crate::models::ScreenableField> get_all_screen_tab_fields(screen_id, tab_id, project_key)
Get all screen tab fields

Returns all fields for a screen tab.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *  *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) when the project key is specified, providing that the screen is associated with the project through a Screen Scheme and Issue Type Screen Scheme.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |
**project_key** | Option<**String**> | The key of the project. |  |

### Return type

[**Vec<crate::models::ScreenableField>**](ScreenableField.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_screen_tabs

> Vec<crate::models::ScreenableTab> get_all_screen_tabs(screen_id, project_key)
Get all screen tabs

Returns the list of tabs for a screen.  **[Permissions](#permissions) required:**   *  *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).  *  *Administer projects* [project permission](https://confluence.atlassian.com/x/yodKLg) when the project key is specified, providing that the screen is associated with the project through a Screen Scheme and Issue Type Screen Scheme.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**project_key** | Option<**String**> | The key of the project. |  |

### Return type

[**Vec<crate::models::ScreenableTab>**](ScreenableTab.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_screens

> crate::models::PageBeanScreen get_all_screens(start_at, max_results)
Get all screens

Returns all screens.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 100]

### Return type

[**crate::models::PageBeanScreen**](PageBeanScreen.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_available_screen_fields

> Vec<crate::models::ScreenableField> get_available_screen_fields(screen_id)
Get available screen fields

Returns the fields that can be added to a tab on a screen.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |

### Return type

[**Vec<crate::models::ScreenableField>**](ScreenableField.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_type_screen_scheme_mappings

> crate::models::PageBeanIssueTypeScreenSchemeItem get_issue_type_screen_scheme_mappings(start_at, max_results, issue_type_screen_scheme_id)
Get issue type screen scheme items

Returns a list of issue type screen scheme items.  Only issue type screen schemes used in classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**issue_type_screen_scheme_id** | Option<[**Vec<i64>**](i64.md)> | The list of issue type screen scheme IDs. To include multiple issue type screen schemes, separate IDs with ampersand: `issueTypeScreenSchemeId=10000&issueTypeScreenSchemeId=10001`. |  |

### Return type

[**crate::models::PageBeanIssueTypeScreenSchemeItem**](PageBeanIssueTypeScreenSchemeItem.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_issue_type_screen_scheme_project_associations

> crate::models::PageBeanIssueTypeScreenSchemesProjects get_issue_type_screen_scheme_project_associations(start_at, max_results, project_id)
Get issue type screen schemes for projects

Returns a list of issue type screen schemes and, for each issue type screen scheme, a list of the projects that use it.  Only issue type screen schemes used in classic projects are returned.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 50]
**project_id** | Option<[**Vec<i64>**](i64.md)> | The list of project IDs. To include multiple projects, separate IDs with ampersand: `projectId=10000&projectId=10001`. |  |

### Return type

[**crate::models::PageBeanIssueTypeScreenSchemesProjects**](PageBeanIssueTypeScreenSchemesProjects.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_screens_for_field

> crate::models::PageBeanScreen get_screens_for_field(field_id, start_at, max_results)
Get screens for a field

Returns a [paginated](#pagination) list of the screens a field is used in.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**field_id** | **String** | The ID of the field to return screens for. | [required] |
**start_at** | Option<**i64**> | The index of the first item to return in a page of results (page offset). |  |[default to 0]
**max_results** | Option<**i32**> | The maximum number of items to return per page. |  |[default to 100]

### Return type

[**crate::models::PageBeanScreen**](PageBeanScreen.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_screen_tab_field_post

> serde_json::Value move_screen_tab_field_post(screen_id, tab_id, id, move_field_bean)
Move screen tab field

Moves a screen tab field.  If `after` and `position` are provided in the request, `position` is ignored.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |
**id** | **String** | The ID of the field. | [required] |
**move_field_bean** | [**MoveFieldBean**](MoveFieldBean.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_screen_tab_post

> serde_json::Value move_screen_tab_post(screen_id, tab_id, pos)
Move screen tab

Moves a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |
**pos** | **i32** | The position of tab. The base index is 0. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_screen_tab_field_delete

> remove_screen_tab_field_delete(screen_id, tab_id, id)
Remove screen tab field

Removes a field from a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |
**id** | **String** | The ID of the field. | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rename_screen_tab_put

> crate::models::ScreenableTab rename_screen_tab_put(screen_id, tab_id, screenable_tab)
Update screen tab

Updates the name of a screen tab.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**screen_id** | **i64** | The ID of the screen. | [required] |
**tab_id** | **i64** | The ID of the screen tab. | [required] |
**screenable_tab** | [**ScreenableTab**](ScreenableTab.md) |  | [required] |

### Return type

[**crate::models::ScreenableTab**](ScreenableTab.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

