/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// DashboardRequest : Details of a dashboard.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardRequest {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the dashboard.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The details of any share permissions for the dashboard.
    #[serde(rename = "sharePermissions", skip_serializing_if = "Option::is_none")]
    pub share_permissions: Option<Vec<crate::models::SharePermission>>,
}

impl DashboardRequest {
    /// Details of a dashboard.
    pub fn new() -> DashboardRequest {
        DashboardRequest {
            description: None,
            name: None,
            share_permissions: None,
        }
    }
}