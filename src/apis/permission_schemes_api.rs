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

pub struct PermissionSchemesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl PermissionSchemesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> PermissionSchemesApiClient {
        PermissionSchemesApiClient { configuration }
    }
}

pub trait PermissionSchemesApi {
    fn create_permission_grant_post(
        &self,
        scheme_id: i64,
        permission_grant: crate::models::PermissionGrant,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionGrant, Error>;
    fn create_permission_scheme_post(
        &self,
        request_body: ::std::collections::HashMap<String, serde_json::Value>,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionScheme, Error>;
    fn delete_permission_scheme(&self, scheme_id: i64) -> Result<(), Error>;
    fn delete_permission_scheme_entity(
        &self,
        scheme_id: i64,
        permission_id: i64,
    ) -> Result<(), Error>;
    fn get_all_permission_schemes(
        &self,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionSchemes, Error>;
    fn get_permission_scheme(
        &self,
        scheme_id: i64,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionScheme, Error>;
    fn get_permission_scheme_grant(
        &self,
        scheme_id: i64,
        permission_id: i64,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionGrant, Error>;
    fn get_permission_scheme_grants(
        &self,
        scheme_id: i64,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionGrants, Error>;
    fn update_permission_scheme_put(
        &self,
        scheme_id: i64,
        request_body: ::std::collections::HashMap<String, serde_json::Value>,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionScheme, Error>;
}

impl PermissionSchemesApi for PermissionSchemesApiClient {
    fn create_permission_grant_post(
        &self,
        scheme_id: i64,
        permission_grant: crate::models::PermissionGrant,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionGrant, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/permissionscheme/{schemeId}/permission",
            configuration.base_path,
            schemeId = scheme_id
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref s) = expand {
            req_builder = req_builder.query(&[("expand", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&permission_grant);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn create_permission_scheme_post(
        &self,
        request_body: ::std::collections::HashMap<String, serde_json::Value>,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionScheme, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/permissionscheme", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref s) = expand {
            req_builder = req_builder.query(&[("expand", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&request_body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn delete_permission_scheme(&self, scheme_id: i64) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/permissionscheme/{schemeId}",
            configuration.base_path,
            schemeId = scheme_id
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn delete_permission_scheme_entity(
        &self,
        scheme_id: i64,
        permission_id: i64,
    ) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/permissionscheme/{schemeId}/permission/{permissionId}",
            configuration.base_path,
            schemeId = scheme_id,
            permissionId = permission_id
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn get_all_permission_schemes(
        &self,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionSchemes, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/rest/api/3/permissionscheme", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

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

    fn get_permission_scheme(
        &self,
        scheme_id: i64,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionScheme, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/permissionscheme/{schemeId}",
            configuration.base_path,
            schemeId = scheme_id
        );
        let mut req_builder = client.get(uri_str.as_str());

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

    fn get_permission_scheme_grant(
        &self,
        scheme_id: i64,
        permission_id: i64,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionGrant, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/permissionscheme/{schemeId}/permission/{permissionId}",
            configuration.base_path,
            schemeId = scheme_id,
            permissionId = permission_id
        );
        let mut req_builder = client.get(uri_str.as_str());

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

    fn get_permission_scheme_grants(
        &self,
        scheme_id: i64,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionGrants, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/permissionscheme/{schemeId}/permission",
            configuration.base_path,
            schemeId = scheme_id
        );
        let mut req_builder = client.get(uri_str.as_str());

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

    fn update_permission_scheme_put(
        &self,
        scheme_id: i64,
        request_body: ::std::collections::HashMap<String, serde_json::Value>,
        expand: Option<&str>,
    ) -> Result<crate::models::PermissionScheme, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/permissionscheme/{schemeId}",
            configuration.base_path,
            schemeId = scheme_id
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref s) = expand {
            req_builder = req_builder.query(&[("expand", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&request_body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
