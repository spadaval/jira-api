/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CompoundClause : A JQL query clause that consists of nested clauses. For example, `(labels in (urgent, blocker) OR lastCommentedBy = currentUser()). Note that, where nesting is not defined, the parser nests JQL clauses based on the operator precedence. For example, \"A OR B AND C\" is parsed as \"(A OR B) AND C\". See Setting the precedence of operators for more information about precedence in JQL queries.`

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompoundClause {
    /// The list of nested clauses.
    #[serde(rename = "clauses")]
    pub clauses: Vec<crate::models::JqlQueryClause>,
    /// The operator between the clauses.
    #[serde(rename = "operator")]
    pub operator: Operator,
}

impl CompoundClause {
    /// A JQL query clause that consists of nested clauses. For example, `(labels in (urgent, blocker) OR lastCommentedBy = currentUser()). Note that, where nesting is not defined, the parser nests JQL clauses based on the operator precedence. For example, \"A OR B AND C\" is parsed as \"(A OR B) AND C\". See Setting the precedence of operators for more information about precedence in JQL queries.`
    pub fn new(clauses: Vec<crate::models::JqlQueryClause>, operator: Operator) -> CompoundClause {
        CompoundClause { clauses, operator }
    }
}

/// The operator between the clauses.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
    #[serde(rename = "not")]
    Not,
}
