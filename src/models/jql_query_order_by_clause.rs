/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JqlQueryOrderByClause : Details of the order-by JQL clause.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryOrderByClause {
    /// The list of order-by clause fields and their ordering directives.
    #[serde(rename = "fields")]
    pub fields: Vec<crate::models::JqlQueryOrderByClauseElement>,
}

impl JqlQueryOrderByClause {
    /// Details of the order-by JQL clause.
    pub fn new(fields: Vec<crate::models::JqlQueryOrderByClauseElement>) -> JqlQueryOrderByClause {
        JqlQueryOrderByClause { fields }
    }
}
