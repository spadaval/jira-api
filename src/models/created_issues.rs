/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CreatedIssues : Details about the issues created and the errors for requests that failed.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreatedIssues {
    /// Details of the issues created.
    #[serde(rename = "issues", skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<crate::models::CreatedIssue>>,
    /// Error details for failed issue creation requests.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::BulkOperationErrorResult>>,
}

impl CreatedIssues {
    /// Details about the issues created and the errors for requests that failed.
    pub fn new() -> CreatedIssues {
        CreatedIssues {
            issues: None,
            errors: None,
        }
    }
}
