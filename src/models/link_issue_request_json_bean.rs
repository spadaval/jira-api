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
pub struct LinkIssueRequestJsonBean {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<crate::models::IssueLinkType>,
    #[serde(rename = "inwardIssue", skip_serializing_if = "Option::is_none")]
    pub inward_issue: Option<crate::models::LinkedIssue>,
    #[serde(rename = "outwardIssue", skip_serializing_if = "Option::is_none")]
    pub outward_issue: Option<crate::models::LinkedIssue>,
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<crate::models::Comment>,
}

impl LinkIssueRequestJsonBean {
    pub fn new() -> LinkIssueRequestJsonBean {
        LinkIssueRequestJsonBean {
            _type: None,
            inward_issue: None,
            outward_issue: None,
            comment: None,
        }
    }
}
