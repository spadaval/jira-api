/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueFilterForBulkPropertySet : Bulk operation filter details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IssueFilterForBulkPropertySet {
    /// List of issues to perform the bulk operation on.
    #[serde(rename = "entityIds", skip_serializing_if = "Option::is_none")]
    pub entity_ids: Option<Vec<i64>>,
    /// The value of properties to perform the bulk operation on.
    #[serde(rename = "currentValue", skip_serializing_if = "Option::is_none")]
    pub current_value: Option<serde_json::Value>,
    /// Whether the bulk operation occurs only when the property is present on or absent from an issue.
    #[serde(rename = "hasProperty", skip_serializing_if = "Option::is_none")]
    pub has_property: Option<bool>,
}

impl IssueFilterForBulkPropertySet {
    /// Bulk operation filter details.
    pub fn new() -> IssueFilterForBulkPropertySet {
        IssueFilterForBulkPropertySet {
            entity_ids: None,
            current_value: None,
            has_property: None,
        }
    }
}