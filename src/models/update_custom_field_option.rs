/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateCustomFieldOption : Details of the options to update for a custom field.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateCustomFieldOption {
    /// Details of the options to update.
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<crate::models::RenamedOption>>,
}

impl UpdateCustomFieldOption {
    /// Details of the options to update for a custom field.
    pub fn new() -> UpdateCustomFieldOption {
        UpdateCustomFieldOption { options: None }
    }
}
