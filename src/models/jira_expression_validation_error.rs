/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JiraExpressionValidationError : Details about a syntax error.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JiraExpressionValidationError {
    /// The text line in which the error occurred. If absent, the error pertains to the entire expression.
    #[serde(rename = "line", skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
    /// The text column in which the error occurred. If absent, the error pertains to the entire expression.
    #[serde(rename = "column", skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
    /// The detailed message about the error.
    #[serde(rename = "message")]
    pub message: String,
    /// The error type.
    #[serde(rename = "type")]
    pub _type: Type,
}

impl JiraExpressionValidationError {
    /// Details about a syntax error.
    pub fn new(message: String, _type: Type) -> JiraExpressionValidationError {
        JiraExpressionValidationError {
            line: None,
            column: None,
            message,
            _type,
        }
    }
}

/// The error type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "syntax")]
    Syntax,
    #[serde(rename = "other")]
    Other,
}
