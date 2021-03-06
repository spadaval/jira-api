/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;
use std::rc::Rc;

use reqwest;

use super::{configuration, Error};

pub struct AppPropertiesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl AppPropertiesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> AppPropertiesApiClient {
        AppPropertiesApiClient { configuration }
    }
}

pub trait AppPropertiesApi {
    fn delete_addon_property(&self, addon_key: &str, property_key: &str) -> Result<(), Error>;
    fn get_addon_properties(&self, addon_key: &str) -> Result<crate::models::PropertyKeys, Error>;
    fn get_addon_property(
        &self,
        addon_key: &str,
        property_key: &str,
    ) -> Result<crate::models::EntityProperty, Error>;
    fn put_addon_property(
        &self,
        addon_key: &str,
        property_key: &str,
        body: serde_json::Value,
    ) -> Result<crate::models::OperationMessage, Error>;
}

impl AppPropertiesApi for AppPropertiesApiClient {
    fn delete_addon_property(&self, addon_key: &str, property_key: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey}",
            configuration.base_path,
            addonKey = crate::apis::urlencode(addon_key),
            propertyKey = crate::apis::urlencode(property_key)
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn get_addon_properties(&self, addon_key: &str) -> Result<crate::models::PropertyKeys, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/atlassian-connect/1/addons/{addonKey}/properties",
            configuration.base_path,
            addonKey = crate::apis::urlencode(addon_key)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_addon_property(
        &self,
        addon_key: &str,
        property_key: &str,
    ) -> Result<crate::models::EntityProperty, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey}",
            configuration.base_path,
            addonKey = crate::apis::urlencode(addon_key),
            propertyKey = crate::apis::urlencode(property_key)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn put_addon_property(
        &self,
        addon_key: &str,
        property_key: &str,
        body: serde_json::Value,
    ) -> Result<crate::models::OperationMessage, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/atlassian-connect/1/addons/{addonKey}/properties/{propertyKey}",
            configuration.base_path,
            addonKey = crate::apis::urlencode(addon_key),
            propertyKey = crate::apis::urlencode(property_key)
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
