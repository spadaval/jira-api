/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WebhookDetails : A list of webhooks.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookDetails {
    /// The JQL filter that specifies which issues the webhook is sent for. Only a subset of JQL can be used. The supported elements are:   *  Fields: `issueKey`, `project`, `issuetype`, `status`, `assignee`, `reporter`, `issue.property`, and `cf[id]` (for custom fields—only the epic label custom field is supported).  *  Operators: `=`, `!=`, `IN`, and `NOT IN`.
    #[serde(rename = "jqlFilter")]
    pub jql_filter: String,
    /// The Jira events that trigger the webhook.
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Events>,
}

impl WebhookDetails {
    /// A list of webhooks.
    pub fn new(jql_filter: String) -> WebhookDetails {
        WebhookDetails {
            jql_filter,
            events: None,
        }
    }
}

/// The Jira events that trigger the webhook.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Events {
    #[serde(rename = "jira:issue_created")]
    JiraissueCreated,
    #[serde(rename = "jira:issue_updated")]
    JiraissueUpdated,
    #[serde(rename = "jira:issue_deleted")]
    JiraissueDeleted,
    #[serde(rename = "comment_created")]
    CommentCreated,
    #[serde(rename = "comment_updated")]
    CommentUpdated,
    #[serde(rename = "comment_deleted")]
    CommentDeleted,
    #[serde(rename = "issue_property_set")]
    IssuePropertySet,
    #[serde(rename = "issue_property_deleted")]
    IssuePropertyDeleted,
}
