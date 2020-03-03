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

pub struct IssueAttachmentsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl IssueAttachmentsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> IssueAttachmentsApiClient {
        IssueAttachmentsApiClient { configuration }
    }
}

pub trait IssueAttachmentsApi {
    fn add_attachment_post(
        &self,
        issue_id_or_key: &str,
    ) -> Result<Vec<crate::models::Attachment>, Error>;
    fn expand_attachment_for_humans_get(
        &self,
        id: &str,
    ) -> Result<crate::models::AttachmentArchiveMetadataReadable, Error>;
    fn expand_attachment_for_machines_get(
        &self,
        id: &str,
    ) -> Result<crate::models::AttachmentArchiveImpl, Error>;
    fn get_attachment(&self, id: &str) -> Result<crate::models::AttachmentMetadata, Error>;
    fn get_attachment_meta(&self) -> Result<crate::models::AttachmentSettings, Error>;
    fn remove_attachment_delete(&self, id: &str) -> Result<(), Error>;
}

impl IssueAttachmentsApi for IssueAttachmentsApiClient {
    fn add_attachment_post(
        &self,
        issue_id_or_key: &str,
    ) -> Result<Vec<crate::models::Attachment>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/issue/{issueIdOrKey}/attachments",
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

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn expand_attachment_for_humans_get(
        &self,
        id: &str,
    ) -> Result<crate::models::AttachmentArchiveMetadataReadable, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/attachment/{id}/expand/human",
            configuration.base_path,
            id = crate::apis::urlencode(id)
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

    fn expand_attachment_for_machines_get(
        &self,
        id: &str,
    ) -> Result<crate::models::AttachmentArchiveImpl, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/attachment/{id}/expand/raw",
            configuration.base_path,
            id = crate::apis::urlencode(id)
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

    fn get_attachment(&self, id: &str) -> Result<crate::models::AttachmentMetadata, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/attachment/{id}",
            configuration.base_path,
            id = crate::apis::urlencode(id)
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

    fn get_attachment_meta(&self) -> Result<crate::models::AttachmentSettings, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/attachment/meta", configuration.base_path);
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

    fn remove_attachment_delete(&self, id: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/attachment/{id}",
            configuration.base_path,
            id = crate::apis::urlencode(id)
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
}