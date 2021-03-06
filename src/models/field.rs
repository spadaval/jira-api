/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Field : Details of a field.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Field {
    /// The ID of the field.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the field.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<crate::models::JsonTypeBean>,
    /// The description of the field.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The key of the field.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Whether the field is locked.
    #[serde(rename = "isLocked", skip_serializing_if = "Option::is_none")]
    pub is_locked: Option<bool>,
    /// Number of screens where the field is used.
    #[serde(rename = "screensCount", skip_serializing_if = "Option::is_none")]
    pub screens_count: Option<i64>,
    /// Number of contexts where the field is used.
    #[serde(rename = "contextsCount", skip_serializing_if = "Option::is_none")]
    pub contexts_count: Option<i64>,
    #[serde(rename = "lastUsed", skip_serializing_if = "Option::is_none")]
    pub last_used: Option<crate::models::FieldLastUsed>,
}

impl Field {
    /// Details of a field.
    pub fn new(id: String, name: String) -> Field {
        Field {
            id,
            name,
            schema: None,
            description: None,
            key: None,
            is_locked: None,
            screens_count: None,
            contexts_count: None,
            last_used: None,
        }
    }
}
