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
pub struct BulkOperationErrorResult {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "elementErrors", skip_serializing_if = "Option::is_none")]
    pub element_errors: Option<crate::models::ErrorCollection>,
    #[serde(
        rename = "failedElementNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub failed_element_number: Option<i32>,
}

impl BulkOperationErrorResult {
    pub fn new() -> BulkOperationErrorResult {
        BulkOperationErrorResult {
            status: None,
            element_errors: None,
            failed_element_number: None,
        }
    }
}
