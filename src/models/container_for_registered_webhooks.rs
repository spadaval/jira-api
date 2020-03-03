/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ContainerForRegisteredWebhooks : Container for a list of registered webhooks. Webhook details are returned in the same order as the request.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerForRegisteredWebhooks {
    /// A list of registered webhooks.
    #[serde(
        rename = "webhookRegistrationResult",
        skip_serializing_if = "Option::is_none"
    )]
    pub webhook_registration_result: Option<Vec<crate::models::RegisteredWebhook>>,
}

impl ContainerForRegisteredWebhooks {
    /// Container for a list of registered webhooks. Webhook details are returned in the same order as the request.
    pub fn new() -> ContainerForRegisteredWebhooks {
        ContainerForRegisteredWebhooks {
            webhook_registration_result: None,
        }
    }
}
