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
pub struct CreateUpdateRoleRequestBean {
    /// The name of the project role. Must be unique. Cannot begin or end with whitespace. The maximum length is 255 characters. Required when creating a project role. Optional when partially updating a project role.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A description of the project role. Required when fully updating a project role. Optional when creating or partially updating a project role.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateUpdateRoleRequestBean {
    pub fn new() -> CreateUpdateRoleRequestBean {
        CreateUpdateRoleRequestBean {
            name: None,
            description: None,
        }
    }
}
