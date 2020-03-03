/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// FoundUsersAndGroups : List of users and groups found in a search.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FoundUsersAndGroups {
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<crate::models::FoundUsers>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<crate::models::FoundGroups>,
}

impl FoundUsersAndGroups {
    /// List of users and groups found in a search.
    pub fn new() -> FoundUsersAndGroups {
        FoundUsersAndGroups {
            users: None,
            groups: None,
        }
    }
}
