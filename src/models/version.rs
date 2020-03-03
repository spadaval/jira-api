/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Version : Details about a project version.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Version {
    /// Use [expand](em>#expansion) to include additional information about version in the response. This parameter accepts a comma-separated list. Expand options include:   *  `operations` Returns the list of operations available for this version.  *  `issuesstatus` Returns the count of issues in this version for each of the status categories *to do*, *in progress*, *done*, and *unmapped*. The *unmapped* property contains a count of issues with a status other than *to do*, *in progress*, and *done*.  Optional for create and update.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The URL of the version.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// The ID of the version.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The description of the version. Optional when creating or updating a version.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The unique name of the version. Required when creating a version. Optional when updating a version. The maximum length is 255 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Indicates that the version is archived. Optional when creating or updating a version.
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Indicates that the version is released. If the version is released a request to release again is ignored. Not applicable when creating a version. Optional when updating a version.
    #[serde(rename = "released", skip_serializing_if = "Option::is_none")]
    pub released: Option<bool>,
    /// The start date of the version. Expressed in ISO 8601 format (yyyy-mm-dd). Optional when creating or updating a version.
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// The release date of the version. Expressed in ISO 8601 format (yyyy-mm-dd). Optional when creating or updating a version.
    #[serde(rename = "releaseDate", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    /// Indicates that the version is overdue.
    #[serde(rename = "overdue", skip_serializing_if = "Option::is_none")]
    pub overdue: Option<bool>,
    /// The date on which work on this version is expected to start, expressed in the instance's *Day/Month/Year Format* date format.
    #[serde(rename = "userStartDate", skip_serializing_if = "Option::is_none")]
    pub user_start_date: Option<String>,
    /// The date on which work on this version is expected to finish, expressed in the instance's *Day/Month/Year Format* date format.
    #[serde(rename = "userReleaseDate", skip_serializing_if = "Option::is_none")]
    pub user_release_date: Option<String>,
    /// Deprecated. Use `projectId`.
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// The ID of the project to which this version is attached. Required when creating a version. Not applicable when updating a version.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    /// The URL of the self link to the version to which all unfixed issues are moved when a version is released. Not applicable when creating a version. Optional when updating a version.
    #[serde(
        rename = "moveUnfixedIssuesTo",
        skip_serializing_if = "Option::is_none"
    )]
    pub move_unfixed_issues_to: Option<String>,
    /// If the expand option `operations` is used, returns the list of operations available for this version.
    #[serde(rename = "operations", skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<crate::models::SimpleLink>>,
    /// If the expand option `issuesstatus` is used, returns the count of issues in this version for each of the status categories *to do*, *in progress*, *done*, and *unmapped*. The *unmapped* property contains a count of issues with a status other than *to do*, *in progress*, and *done*.
    #[serde(
        rename = "issuesStatusForFixVersion",
        skip_serializing_if = "Option::is_none"
    )]
    pub issues_status_for_fix_version: Option<crate::models::VersionIssuesStatus>,
}

impl Version {
    /// Details about a project version.
    pub fn new() -> Version {
        Version {
            expand: None,
            _self: None,
            id: None,
            description: None,
            name: None,
            archived: None,
            released: None,
            start_date: None,
            release_date: None,
            overdue: None,
            user_start_date: None,
            user_release_date: None,
            project: None,
            project_id: None,
            move_unfixed_issues_to: None,
            operations: None,
            issues_status_for_fix_version: None,
        }
    }
}
