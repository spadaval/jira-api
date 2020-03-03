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

pub struct ProjectRoleActorsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ProjectRoleActorsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ProjectRoleActorsApiClient {
        ProjectRoleActorsApiClient { configuration }
    }
}

pub trait ProjectRoleActorsApi {
    fn add_actor_users_post(
        &self,
        project_id_or_key: &str,
        id: i64,
        actors_map: crate::models::ActorsMap,
    ) -> Result<crate::models::ProjectRole, Error>;
    fn add_project_role_actors_to_role_post(
        &self,
        id: i64,
        actor_input_bean: crate::models::ActorInputBean,
    ) -> Result<crate::models::ProjectRole, Error>;
    fn delete_actor(
        &self,
        project_id_or_key: &str,
        id: i64,
        user: Option<&str>,
        group: Option<&str>,
    ) -> Result<(), Error>;
    fn delete_project_role_actors_from_role(
        &self,
        id: i64,
        user: Option<&str>,
        group: Option<&str>,
    ) -> Result<crate::models::ProjectRole, Error>;
    fn get_project_role_actors_for_role(
        &self,
        id: i64,
    ) -> Result<crate::models::ProjectRole, Error>;
    fn set_actors_put(
        &self,
        project_id_or_key: &str,
        id: i64,
        project_role_actors_update_bean: crate::models::ProjectRoleActorsUpdateBean,
    ) -> Result<crate::models::ProjectRole, Error>;
}

impl ProjectRoleActorsApi for ProjectRoleActorsApiClient {
    fn add_actor_users_post(
        &self,
        project_id_or_key: &str,
        id: i64,
        actors_map: crate::models::ActorsMap,
    ) -> Result<crate::models::ProjectRole, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/project/{projectIdOrKey}/role/{id}",
            configuration.base_path,
            projectIdOrKey = crate::apis::urlencode(project_id_or_key),
            id = id
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
        req_builder = req_builder.json(&actors_map);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn add_project_role_actors_to_role_post(
        &self,
        id: i64,
        actor_input_bean: crate::models::ActorInputBean,
    ) -> Result<crate::models::ProjectRole, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/role/{id}/actors",
            configuration.base_path,
            id = id
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&actor_input_bean);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn delete_actor(
        &self,
        project_id_or_key: &str,
        id: i64,
        user: Option<&str>,
        group: Option<&str>,
    ) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/project/{projectIdOrKey}/role/{id}",
            configuration.base_path,
            projectIdOrKey = crate::apis::urlencode(project_id_or_key),
            id = id
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref s) = user {
            req_builder = req_builder.query(&[("user", &s.to_string())]);
        }
        if let Some(ref s) = group {
            req_builder = req_builder.query(&[("group", &s.to_string())]);
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

    fn delete_project_role_actors_from_role(
        &self,
        id: i64,
        user: Option<&str>,
        group: Option<&str>,
    ) -> Result<crate::models::ProjectRole, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/role/{id}/actors",
            configuration.base_path,
            id = id
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref s) = user {
            req_builder = req_builder.query(&[("user", &s.to_string())]);
        }
        if let Some(ref s) = group {
            req_builder = req_builder.query(&[("group", &s.to_string())]);
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

    fn get_project_role_actors_for_role(
        &self,
        id: i64,
    ) -> Result<crate::models::ProjectRole, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/role/{id}/actors",
            configuration.base_path,
            id = id
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

    fn set_actors_put(
        &self,
        project_id_or_key: &str,
        id: i64,
        project_role_actors_update_bean: crate::models::ProjectRoleActorsUpdateBean,
    ) -> Result<crate::models::ProjectRole, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/rest/api/3/project/{projectIdOrKey}/role/{id}",
            configuration.base_path,
            projectIdOrKey = crate::apis::urlencode(project_id_or_key),
            id = id
        );
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };
        req_builder = req_builder.json(&project_role_actors_update_bean);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
