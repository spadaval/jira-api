/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeScreenSchemesProjects : List of issue type screen schemes, each with a list of the projects that use it.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemesProjects {
    /// Details of an issue type screen scheme.
    #[serde(
        rename = "issueTypeScreenScheme",
        skip_serializing_if = "Option::is_none"
    )]
    pub issue_type_screen_scheme: Option<crate::models::IssueTypeScreenScheme>,
    /// The IDs of the projects using the issue type screen scheme.
    #[serde(rename = "projectIds")]
    pub project_ids: Vec<String>,
}

impl IssueTypeScreenSchemesProjects {
    /// List of issue type screen schemes, each with a list of the projects that use it.
    pub fn new(project_ids: Vec<String>) -> IssueTypeScreenSchemesProjects {
        IssueTypeScreenSchemesProjects {
            issue_type_screen_scheme: None,
            project_ids,
        }
    }
}
