/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectRoleGroupBean : Details of the role actor group.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectRoleGroupBean {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ProjectRoleGroupBean {
    /// Details of the role actor group.
    pub fn new() -> ProjectRoleGroupBean {
        ProjectRoleGroupBean {
            display_name: None,
            name: None,
        }
    }
}
