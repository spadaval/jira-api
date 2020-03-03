/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ChangeDetails : A change item.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeDetails {
    /// The name of the field changed.
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// The type of the field changed.
    #[serde(rename = "fieldtype", skip_serializing_if = "Option::is_none")]
    pub fieldtype: Option<String>,
    /// The ID of the field changed.
    #[serde(rename = "fieldId", skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    /// The details of the original value.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The details of the original value as a string.
    #[serde(rename = "fromString", skip_serializing_if = "Option::is_none")]
    pub from_string: Option<String>,
    /// The details of the new value.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// The details of the new value as a string.
    #[serde(rename = "toString", skip_serializing_if = "Option::is_none")]
    pub to_string: Option<String>,
}

impl ChangeDetails {
    /// A change item.
    pub fn new() -> ChangeDetails {
        ChangeDetails {
            field: None,
            fieldtype: None,
            field_id: None,
            from: None,
            from_string: None,
            to: None,
            to_string: None,
        }
    }
}
