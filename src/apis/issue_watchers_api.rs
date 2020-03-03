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

pub struct IssueWatchersApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl IssueWatchersApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> IssueWatchersApiClient {
        IssueWatchersApiClient { configuration }
    }
}

pub trait IssueWatchersApi {
    fn add_watcher_post(
        &self,
        issue_id_or_key: &str,
        body: &str,
    ) -> Result<serde_json::Value, Error>;
    fn get_issue_watchers(&self, issue_id_or_key: &str) -> Result<crate::models::Watchers, Error>;
    fn remove_watcher_delete(
        &self,
        issue_id_or_key: &str,
        username: Option<&str>,
        account_id: Option<&str>,
    ) -> Result<(), Error>;
}

impl IssueWatchersApi for IssueWatchersApiClient {
    fn add_watcher_post(
        &self,
        issue_id_or_key: &str,
        body: &str,
    ) -> Result<serde_json::Value, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/issue/{issueIdOrKey}/watchers",
            configuration.base_path,
            issueIdOrKey = crate::apis::urlencode(issue_id_or_key)
        );
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
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_issue_watchers(&self, issue_id_or_key: &str) -> Result<crate::models::Watchers, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/issue/{issueIdOrKey}/watchers",
            configuration.base_path,
            issueIdOrKey = crate::apis::urlencode(issue_id_or_key)
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

    fn remove_watcher_delete(
        &self,
        issue_id_or_key: &str,
        username: Option<&str>,
        account_id: Option<&str>,
    ) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/issue/{issueIdOrKey}/watchers",
            configuration.base_path,
            issueIdOrKey = crate::apis::urlencode(issue_id_or_key)
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref s) = username {
            req_builder = req_builder.query(&[("username", &s.to_string())]);
        }
        if let Some(ref s) = account_id {
            req_builder = req_builder.query(&[("accountId", &s.to_string())]);
        }
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
}