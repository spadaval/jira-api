/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Avatars : Details about system and custom avatars.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Avatars {
    /// System avatars list.
    #[serde(rename = "system", skip_serializing_if = "Option::is_none")]
    pub system: Option<Vec<crate::models::Avatar>>,
    /// Custom avatars list.
    #[serde(rename = "custom", skip_serializing_if = "Option::is_none")]
    pub custom: Option<Vec<crate::models::Avatar>>,
}

impl Avatars {
    /// Details about system and custom avatars.
    pub fn new() -> Avatars {
        Avatars {
            system: None,
            custom: None,
        }
    }
}
