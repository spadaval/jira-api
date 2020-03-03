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

pub struct IssueFieldsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl IssueFieldsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> IssueFieldsApiClient {
        IssueFieldsApiClient { configuration }
    }
}

pub trait IssueFieldsApi {
    fn create_custom_field_post(
        &self,
        custom_field_definition_json_bean: crate::models::CustomFieldDefinitionJsonBean,
    ) -> Result<crate::models::FieldDetails, Error>;
    fn get_contexts_for_field(
        &self,
        field_id: &str,
        start_at: Option<i64>,
        max_results: Option<i32>,
    ) -> Result<crate::models::PageBeanContext, Error>;
    fn get_fields(&self) -> Result<Vec<crate::models::FieldDetails>, Error>;
    fn get_fields_paginated(
        &self,
        start_at: Option<i64>,
        max_results: Option<i32>,
        _type: Option<Vec<String>>,
        id: Option<Vec<String>>,
        query: Option<&str>,
        order_by: Option<&str>,
        expand: Option<&str>,
    ) -> Result<crate::models::PageBeanField, Error>;
}

impl IssueFieldsApi for IssueFieldsApiClient {
    fn create_custom_field_post(
        &self,
        custom_field_definition_json_bean: crate::models::CustomFieldDefinitionJsonBean,
    ) -> Result<crate::models::FieldDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/field", configuration.base_path);
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
        req_builder = req_builder.json(&custom_field_definition_json_bean);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_contexts_for_field(
        &self,
        field_id: &str,
        start_at: Option<i64>,
        max_results: Option<i32>,
    ) -> Result<crate::models::PageBeanContext, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/field/{fieldId}/contexts",
            configuration.base_path,
            fieldId = crate::apis::urlencode(field_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = start_at {
            req_builder = req_builder.query(&[("startAt", &s.to_string())]);
        }
        if let Some(ref s) = max_results {
            req_builder = req_builder.query(&[("maxResults", &s.to_string())]);
        }
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

    fn get_fields(&self) -> Result<Vec<crate::models::FieldDetails>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/field", configuration.base_path);
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

    fn get_fields_paginated(
        &self,
        start_at: Option<i64>,
        max_results: Option<i32>,
        _type: Option<Vec<String>>,
        id: Option<Vec<String>>,
        query: Option<&str>,
        order_by: Option<&str>,
        expand: Option<&str>,
    ) -> Result<crate::models::PageBeanField, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/field/search", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = start_at {
            req_builder = req_builder.query(&[("startAt", &s.to_string())]);
        }
        if let Some(ref s) = max_results {
            req_builder = req_builder.query(&[("maxResults", &s.to_string())]);
        }
        if let Some(ref s) = _type {
            req_builder = req_builder.query(&[(
                "type",
                &s.into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        }
        if let Some(ref s) = id {
            req_builder = req_builder.query(&[(
                "id",
                &s.into_iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
                    .to_string(),
            )]);
        }
        if let Some(ref s) = query {
            req_builder = req_builder.query(&[("query", &s.to_string())]);
        }
        if let Some(ref s) = order_by {
            req_builder = req_builder.query(&[("orderBy", &s.to_string())]);
        }
        if let Some(ref s) = expand {
            req_builder = req_builder.query(&[("expand", &s.to_string())]);
        }
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
}