/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssuesUpdateBean {
    #[serde(rename = "issueUpdates", skip_serializing_if = "Option::is_none")]
    pub issue_updates: Option<Vec<crate::models::IssueUpdateDetails>>,
}

impl IssuesUpdateBean {
    pub fn new() -> IssuesUpdateBean {
        IssuesUpdateBean {
            issue_updates: None,
        }
    }
}
