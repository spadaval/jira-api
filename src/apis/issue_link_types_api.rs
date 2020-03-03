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

pub struct IssueLinkTypesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl IssueLinkTypesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> IssueLinkTypesApiClient {
        IssueLinkTypesApiClient { configuration }
    }
}

pub trait IssueLinkTypesApi {
    fn create_issue_link_type_post(
        &self,
        issue_link_type: crate::models::IssueLinkType,
    ) -> Result<crate::models::IssueLinkType, Error>;
    fn delete_issue_link_type(&self, issue_link_type_id: &str) -> Result<(), Error>;
    fn get_issue_link_type(
        &self,
        issue_link_type_id: &str,
    ) -> Result<crate::models::IssueLinkType, Error>;
    fn get_issue_link_types(&self) -> Result<crate::models::IssueLinkTypes, Error>;
    fn update_issue_link_type_put(
        &self,
        issue_link_type_id: &str,
        issue_link_type: crate::models::IssueLinkType,
    ) -> Result<crate::models::IssueLinkType, Error>;
}

impl IssueLinkTypesApi for IssueLinkTypesApiClient {
    fn create_issue_link_type_post(
        &self,
        issue_link_type: crate::models::IssueLinkType,
    ) -> Result<crate::models::IssueLinkType, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/issueLinkType", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&issue_link_type);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn delete_issue_link_type(&self, issue_link_type_id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/issueLinkType/{issueLinkTypeId}",
            configuration.base_path,
            issueLinkTypeId = crate::apis::urlencode(issue_link_type_id)
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn get_issue_link_type(
        &self,
        issue_link_type_id: &str,
    ) -> Result<crate::models::IssueLinkType, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/issueLinkType/{issueLinkTypeId}",
            configuration.base_path,
            issueLinkTypeId = crate::apis::urlencode(issue_link_type_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_issue_link_types(&self) -> Result<crate::models::IssueLinkTypes, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/issueLinkType", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn update_issue_link_type_put(
        &self,
        issue_link_type_id: &str,
        issue_link_type: crate::models::IssueLinkType,
    ) -> Result<crate::models::IssueLinkType, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/issueLinkType/{issueLinkTypeId}",
            configuration.base_path,
            issueLinkTypeId = crate::apis::urlencode(issue_link_type_id)
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&issue_link_type);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}