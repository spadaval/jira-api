/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// FieldWasClause : A clause that asserts a previous value of a field. For example, `status WAS \"Resolved\" BY currentUser() BEFORE \"2019/02/02\"`. See [WAS](https://confluence.atlassian.com/x/dgiiLQ#Advancedsearching-operatorsreference-WASWAS) for more information about the WAS operator.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldWasClause {
    #[serde(rename = "field")]
    pub field: crate::models::JqlQueryField,
    /// The operator between the field and operand.
    #[serde(rename = "operator")]
    pub operator: Operator,
    #[serde(rename = "operand")]
    pub operand: crate::models::JqlQueryClauseOperand,
    /// The list of time predicates.
    #[serde(rename = "predicates")]
    pub predicates: Vec<crate::models::JqlQueryClauseTimePredicate>,
}

impl FieldWasClause {
    /// A clause that asserts a previous value of a field. For example, `status WAS \"Resolved\" BY currentUser() BEFORE \"2019/02/02\"`. See [WAS](https://confluence.atlassian.com/x/dgiiLQ#Advancedsearching-operatorsreference-WASWAS) for more information about the WAS operator.
    pub fn new(
        field: crate::models::JqlQueryField,
        operator: Operator,
        operand: crate::models::JqlQueryClauseOperand,
        predicates: Vec<crate::models::JqlQueryClauseTimePredicate>,
    ) -> FieldWasClause {
        FieldWasClause {
            field,
            operator,
            operand,
            predicates,
        }
    }
}

/// The operator between the field and operand.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "was")]
    Was,
    #[serde(rename = "was in")]
    Was_in,
    #[serde(rename = "was not in")]
    Was_not_in,
    #[serde(rename = "was not")]
    Was_not,
}
