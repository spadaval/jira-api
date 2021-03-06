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

pub struct TimeTrackingApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl TimeTrackingApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> TimeTrackingApiClient {
        TimeTrackingApiClient { configuration }
    }
}

pub trait TimeTrackingApi {
    fn get_available_time_tracking_implementations(
        &self,
    ) -> Result<Vec<crate::models::TimeTrackingProvider>, Error>;
    fn get_selected_time_tracking_implementation(
        &self,
    ) -> Result<crate::models::TimeTrackingProvider, Error>;
    fn get_shared_time_tracking_configuration(
        &self,
    ) -> Result<crate::models::TimeTrackingConfiguration, Error>;
    fn select_time_tracking_implementation_put(
        &self,
        time_tracking_provider: crate::models::TimeTrackingProvider,
    ) -> Result<serde_json::Value, Error>;
    fn set_shared_time_tracking_configuration_put(
        &self,
        time_tracking_configuration: crate::models::TimeTrackingConfiguration,
    ) -> Result<crate::models::TimeTrackingConfiguration, Error>;
}

impl TimeTrackingApi for TimeTrackingApiClient {
    fn get_available_time_tracking_implementations(
        &self,
    ) -> Result<Vec<crate::models::TimeTrackingProvider>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/configuration/timetracking/list",
            configuration.base_path
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_selected_time_tracking_implementation(
        &self,
    ) -> Result<crate::models::TimeTrackingProvider, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/configuration/timetracking",
            configuration.base_path
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_shared_time_tracking_configuration(
        &self,
    ) -> Result<crate::models::TimeTrackingConfiguration, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/configuration/timetracking/options",
            configuration.base_path
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn select_time_tracking_implementation_put(
        &self,
        time_tracking_provider: crate::models::TimeTrackingProvider,
    ) -> Result<serde_json::Value, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/configuration/timetracking",
            configuration.base_path
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&time_tracking_provider);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn set_shared_time_tracking_configuration_put(
        &self,
        time_tracking_configuration: crate::models::TimeTrackingConfiguration,
    ) -> Result<crate::models::TimeTrackingConfiguration, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/configuration/timetracking/options",
            configuration.base_path
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&time_tracking_configuration);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
