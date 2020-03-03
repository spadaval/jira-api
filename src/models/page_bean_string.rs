/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PageBeanString : A page of items.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PageBeanString {
    /// The URL of the page.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub _self: Option<String>,
    /// If there is another page of results, the URL of the next page.
    #[serde(rename = "nextPage", skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    /// The maximum number of items that could be returned.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The index of the first item returned.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i64>,
    /// The number of items returned.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /// Whether this is the last page.
    #[serde(rename = "isLast", skip_serializing_if = "Option::is_none")]
    pub is_last: Option<bool>,
    /// The list of items.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl PageBeanString {
    /// A page of items.
    pub fn new() -> PageBeanString {
        PageBeanString {
            _self: None,
            next_page: None,
            max_results: None,
            start_at: None,
            total: None,
            is_last: None,
            values: None,
        }
    }
}
