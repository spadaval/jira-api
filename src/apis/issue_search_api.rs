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

pub struct IssueSearchApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl IssueSearchApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> IssueSearchApiClient {
        IssueSearchApiClient { configuration }
    }
}

pub trait IssueSearchApi {
    fn get_issue_picker_resource(
        &self,
        query: Option<&str>,
        current_jql: Option<&str>,
        current_issue_key: Option<&str>,
        current_project_id: Option<&str>,
        show_sub_tasks: Option<bool>,
        show_sub_task_parent: Option<bool>,
    ) -> Result<crate::models::IssuePickerSuggestions, Error>;
    fn match_issues_post(
        &self,
        issues_and_jql_queries: crate::models::IssuesAndJqlQueries,
    ) -> Result<crate::models::IssueMatches, Error>;
    fn search_for_issues_using_jql_get(
        &self,
        jql: Option<&str>,
        start_at: Option<i32>,
        max_results: Option<i32>,
        validate_query: Option<&str>,
        fields: Option<Vec<String>>,
        expand: Option<&str>,
        properties: Option<Vec<String>>,
        fields_by_keys: Option<bool>,
    ) -> Result<crate::models::SearchResults, Error>;
    fn search_for_issues_using_jql_post(
        &self,
        search_request_bean: crate::models::SearchRequestBean,
    ) -> Result<crate::models::SearchResults, Error>;
}

impl IssueSearchApi for IssueSearchApiClient {
    fn get_issue_picker_resource(
        &self,
        query: Option<&str>,
        current_jql: Option<&str>,
        current_issue_key: Option<&str>,
        current_project_id: Option<&str>,
        show_sub_tasks: Option<bool>,
        show_sub_task_parent: Option<bool>,
    ) -> Result<crate::models::IssuePickerSuggestions, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/issue/picker", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = query {
            req_builder = req_builder.query(&[("query", &s.to_string())]);
        }
        if let Some(ref s) = current_jql {
            req_builder = req_builder.query(&[("currentJQL", &s.to_string())]);
        }
        if let Some(ref s) = current_issue_key {
            req_builder = req_builder.query(&[("currentIssueKey", &s.to_string())]);
        }
        if let Some(ref s) = current_project_id {
            req_builder = req_builder.query(&[("currentProjectId", &s.to_string())]);
        }
        if let Some(ref s) = show_sub_tasks {
            req_builder = req_builder.query(&[("showSubTasks", &s.to_string())]);
        }
        if let Some(ref s) = show_sub_task_parent {
            req_builder = req_builder.query(&[("showSubTaskParent", &s.to_string())]);
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

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn match_issues_post(
        &self,
        issues_and_jql_queries: crate::models::IssuesAndJqlQueries,
    ) -> Result<crate::models::IssueMatches, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/jql/match", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&issues_and_jql_queries);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn search_for_issues_using_jql_get(
        &self,
        jql: Option<&str>,
        start_at: Option<i32>,
        max_results: Option<i32>,
        validate_query: Option<&str>,
        fields: Option<Vec<String>>,
        expand: Option<&str>,
        properties: Option<Vec<String>>,
        fields_by_keys: Option<bool>,
    ) -> Result<crate::models::SearchResults, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/search", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = jql {
            req_builder = req_builder.query(&[("jql", &s.to_string())]);
        }
        if let Some(ref s) = start_at {
            req_builder = req_builder.query(&[("startAt", &s.to_string())]);
        }
        if let Some(ref s) = max_results {
            req_builder = req_builder.query(&[("maxResults", &s.to_string())]);
        }
        if let Some(ref s) = validate_query {
            req_builder = req_builder.query(&[("validateQuery", &s.to_string())]);
        }
        if let Some(ref s) = fields {
            req_builder = req_builder.query(&[(
                "fields",
                &s.into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        }
        if let Some(ref s) = expand {
            req_builder = req_builder.query(&[("expand", &s.to_string())]);
        }
        if let Some(ref s) = properties {
            req_builder = req_builder.query(&[(
                "properties",
                &s.into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        }
        if let Some(ref s) = fields_by_keys {
            req_builder = req_builder.query(&[("fieldsByKeys", &s.to_string())]);
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

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn search_for_issues_using_jql_post(
        &self,
        search_request_bean: crate::models::SearchRequestBean,
    ) -> Result<crate::models::SearchResults, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/search", configuration.base_path);
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
        req_builder = req_builder.json(&search_request_bean);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
