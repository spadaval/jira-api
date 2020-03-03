/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Hierarchy : Project Issue Type Hierarchy

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hierarchy {
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<Vec<crate::models::HierarchyLevel>>,
}

impl Hierarchy {
    /// Project Issue Type Hierarchy
    pub fn new() -> Hierarchy {
        Hierarchy { level: None }
    }
}
