/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JqlQuery : A parsed JQL query.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQuery {
    #[serde(rename = "where", skip_serializing_if = "Option::is_none")]
    pub _where: Option<crate::models::JqlQueryClause>,
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<crate::models::JqlQueryOrderByClause>,
}

impl JqlQuery {
    /// A parsed JQL query.
    pub fn new() -> JqlQuery {
        JqlQuery {
            _where: None,
            order_by: None,
        }
    }
}
