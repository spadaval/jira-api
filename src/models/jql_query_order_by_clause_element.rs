/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JqlQueryOrderByClauseElement : An element of the order-by JQL clause.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JqlQueryOrderByClauseElement {
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<crate::models::JqlQueryField>,
    /// The direction in which to order the results.
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
}

impl JqlQueryOrderByClauseElement {
    /// An element of the order-by JQL clause.
    pub fn new() -> JqlQueryOrderByClauseElement {
        JqlQueryOrderByClauseElement {
            field: None,
            direction: None,
        }
    }
}

/// The direction in which to order the results.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
