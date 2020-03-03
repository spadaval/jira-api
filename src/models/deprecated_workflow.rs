/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// DeprecatedWorkflow : Details about a workflow.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeprecatedWorkflow {
    /// The name of the workflow.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the workflow.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The datetime the workflow was last modified.
    #[serde(rename = "lastModifiedDate", skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    /// This property is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    #[serde(rename = "lastModifiedUser", skip_serializing_if = "Option::is_none")]
    pub last_modified_user: Option<String>,
    /// The account ID of the user that last modified the workflow.
    #[serde(
        rename = "lastModifiedUserAccountId",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified_user_account_id: Option<String>,
    /// The number of steps included in the workflow.
    #[serde(rename = "steps", skip_serializing_if = "Option::is_none")]
    pub steps: Option<i32>,
    /// The scope where this workflow applies
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<crate::models::Scope>,
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

impl DeprecatedWorkflow {
    /// Details about a workflow.
    pub fn new() -> DeprecatedWorkflow {
        DeprecatedWorkflow {
            name: None,
            description: None,
            last_modified_date: None,
            last_modified_user: None,
            last_modified_user_account_id: None,
            steps: None,
            scope: None,
            default: None,
        }
    }
}
