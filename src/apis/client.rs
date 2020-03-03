use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    app_properties_api: Box<dyn crate::apis::AppPropertiesApi>,
    application_roles_api: Box<dyn crate::apis::ApplicationRolesApi>,
    audit_records_api: Box<dyn crate::apis::AuditRecordsApi>,
    avatars_api: Box<dyn crate::apis::AvatarsApi>,
    dashboards_api: Box<dyn crate::apis::DashboardsApi>,
    dynamic_modules_api: Box<dyn crate::apis::DynamicModulesApi>,
    filter_sharing_api: Box<dyn crate::apis::FilterSharingApi>,
    filters_api: Box<dyn crate::apis::FiltersApi>,
    group_and_user_picker_api: Box<dyn crate::apis::GroupAndUserPickerApi>,
    groups_api: Box<dyn crate::apis::GroupsApi>,
    issue_attachments_api: Box<dyn crate::apis::IssueAttachmentsApi>,
    issue_comment_properties_api: Box<dyn crate::apis::IssueCommentPropertiesApi>,
    issue_comments_api: Box<dyn crate::apis::IssueCommentsApi>,
    issue_custom_field_options_api: Box<dyn crate::apis::IssueCustomFieldOptionsApi>,
    issue_custom_field_options_apps_api: Box<dyn crate::apis::IssueCustomFieldOptionsAppsApi>,
    issue_field_configurations_api: Box<dyn crate::apis::IssueFieldConfigurationsApi>,
    issue_fields_api: Box<dyn crate::apis::IssueFieldsApi>,
    issue_link_types_api: Box<dyn crate::apis::IssueLinkTypesApi>,
    issue_links_api: Box<dyn crate::apis::IssueLinksApi>,
    issue_navigator_settings_api: Box<dyn crate::apis::IssueNavigatorSettingsApi>,
    issue_notification_schemes_api: Box<dyn crate::apis::IssueNotificationSchemesApi>,
    issue_priorities_api: Box<dyn crate::apis::IssuePrioritiesApi>,
    issue_properties_api: Box<dyn crate::apis::IssuePropertiesApi>,
    issue_remote_links_api: Box<dyn crate::apis::IssueRemoteLinksApi>,
    issue_resolutions_api: Box<dyn crate::apis::IssueResolutionsApi>,
    issue_search_api: Box<dyn crate::apis::IssueSearchApi>,
    issue_security_level_api: Box<dyn crate::apis::IssueSecurityLevelApi>,
    issue_security_schemes_api: Box<dyn crate::apis::IssueSecuritySchemesApi>,
    issue_type_properties_api: Box<dyn crate::apis::IssueTypePropertiesApi>,
    issue_types_api: Box<dyn crate::apis::IssueTypesApi>,
    issue_votes_api: Box<dyn crate::apis::IssueVotesApi>,
    issue_watchers_api: Box<dyn crate::apis::IssueWatchersApi>,
    issue_worklog_properties_api: Box<dyn crate::apis::IssueWorklogPropertiesApi>,
    issue_worklogs_api: Box<dyn crate::apis::IssueWorklogsApi>,
    issues_api: Box<dyn crate::apis::IssuesApi>,
    jql_api: Box<dyn crate::apis::JQLApi>,
    jira_expressions_api: Box<dyn crate::apis::JiraExpressionsApi>,
    jira_settings_api: Box<dyn crate::apis::JiraSettingsApi>,
    labels_api: Box<dyn crate::apis::LabelsApi>,
    myself_api: Box<dyn crate::apis::MyselfApi>,
    permission_schemes_api: Box<dyn crate::apis::PermissionSchemesApi>,
    permissions_api: Box<dyn crate::apis::PermissionsApi>,
    project_avatars_api: Box<dyn crate::apis::ProjectAvatarsApi>,
    project_categories_api: Box<dyn crate::apis::ProjectCategoriesApi>,
    project_components_api: Box<dyn crate::apis::ProjectComponentsApi>,
    project_key_and_name_validation_api: Box<dyn crate::apis::ProjectKeyAndNameValidationApi>,
    project_permission_schemes_api: Box<dyn crate::apis::ProjectPermissionSchemesApi>,
    project_properties_api: Box<dyn crate::apis::ProjectPropertiesApi>,
    project_role_actors_api: Box<dyn crate::apis::ProjectRoleActorsApi>,
    project_roles_api: Box<dyn crate::apis::ProjectRolesApi>,
    project_types_api: Box<dyn crate::apis::ProjectTypesApi>,
    project_versions_api: Box<dyn crate::apis::ProjectVersionsApi>,
    projects_api: Box<dyn crate::apis::ProjectsApi>,
    screens_api: Box<dyn crate::apis::ScreensApi>,
    server_info_api: Box<dyn crate::apis::ServerInfoApi>,
    tasks_api: Box<dyn crate::apis::TasksApi>,
    time_tracking_api: Box<dyn crate::apis::TimeTrackingApi>,
    user_properties_api: Box<dyn crate::apis::UserPropertiesApi>,
    user_search_api: Box<dyn crate::apis::UserSearchApi>,
    users_api: Box<dyn crate::apis::UsersApi>,
    webhooks_api: Box<dyn crate::apis::WebhooksApi>,
    workflow_scheme_drafts_api: Box<dyn crate::apis::WorkflowSchemeDraftsApi>,
    workflow_scheme_project_associations_api:
        Box<dyn crate::apis::WorkflowSchemeProjectAssociationsApi>,
    workflow_schemes_api: Box<dyn crate::apis::WorkflowSchemesApi>,
    workflow_status_categories_api: Box<dyn crate::apis::WorkflowStatusCategoriesApi>,
    workflow_statuses_api: Box<dyn crate::apis::WorkflowStatusesApi>,
    workflow_transition_properties_api: Box<dyn crate::apis::WorkflowTransitionPropertiesApi>,
    workflow_transition_rules_api: Box<dyn crate::apis::WorkflowTransitionRulesApi>,
    workflows_api: Box<dyn crate::apis::WorkflowsApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            app_properties_api: Box::new(crate::apis::AppPropertiesApiClient::new(rc.clone())),
            application_roles_api: Box::new(crate::apis::ApplicationRolesApiClient::new(
                rc.clone(),
            )),
            audit_records_api: Box::new(crate::apis::AuditRecordsApiClient::new(rc.clone())),
            avatars_api: Box::new(crate::apis::AvatarsApiClient::new(rc.clone())),
            dashboards_api: Box::new(crate::apis::DashboardsApiClient::new(rc.clone())),
            dynamic_modules_api: Box::new(crate::apis::DynamicModulesApiClient::new(rc.clone())),
            filter_sharing_api: Box::new(crate::apis::FilterSharingApiClient::new(rc.clone())),
            filters_api: Box::new(crate::apis::FiltersApiClient::new(rc.clone())),
            group_and_user_picker_api: Box::new(crate::apis::GroupAndUserPickerApiClient::new(
                rc.clone(),
            )),
            groups_api: Box::new(crate::apis::GroupsApiClient::new(rc.clone())),
            issue_attachments_api: Box::new(crate::apis::IssueAttachmentsApiClient::new(
                rc.clone(),
            )),
            issue_comment_properties_api: Box::new(
                crate::apis::IssueCommentPropertiesApiClient::new(rc.clone()),
            ),
            issue_comments_api: Box::new(crate::apis::IssueCommentsApiClient::new(rc.clone())),
            issue_custom_field_options_api: Box::new(
                crate::apis::IssueCustomFieldOptionsApiClient::new(rc.clone()),
            ),
            issue_custom_field_options_apps_api: Box::new(
                crate::apis::IssueCustomFieldOptionsAppsApiClient::new(rc.clone()),
            ),
            issue_field_configurations_api: Box::new(
                crate::apis::IssueFieldConfigurationsApiClient::new(rc.clone()),
            ),
            issue_fields_api: Box::new(crate::apis::IssueFieldsApiClient::new(rc.clone())),
            issue_link_types_api: Box::new(crate::apis::IssueLinkTypesApiClient::new(rc.clone())),
            issue_links_api: Box::new(crate::apis::IssueLinksApiClient::new(rc.clone())),
            issue_navigator_settings_api: Box::new(
                crate::apis::IssueNavigatorSettingsApiClient::new(rc.clone()),
            ),
            issue_notification_schemes_api: Box::new(
                crate::apis::IssueNotificationSchemesApiClient::new(rc.clone()),
            ),
            issue_priorities_api: Box::new(crate::apis::IssuePrioritiesApiClient::new(rc.clone())),
            issue_properties_api: Box::new(crate::apis::IssuePropertiesApiClient::new(rc.clone())),
            issue_remote_links_api: Box::new(crate::apis::IssueRemoteLinksApiClient::new(
                rc.clone(),
            )),
            issue_resolutions_api: Box::new(crate::apis::IssueResolutionsApiClient::new(
                rc.clone(),
            )),
            issue_search_api: Box::new(crate::apis::IssueSearchApiClient::new(rc.clone())),
            issue_security_level_api: Box::new(crate::apis::IssueSecurityLevelApiClient::new(
                rc.clone(),
            )),
            issue_security_schemes_api: Box::new(crate::apis::IssueSecuritySchemesApiClient::new(
                rc.clone(),
            )),
            issue_type_properties_api: Box::new(crate::apis::IssueTypePropertiesApiClient::new(
                rc.clone(),
            )),
            issue_types_api: Box::new(crate::apis::IssueTypesApiClient::new(rc.clone())),
            issue_votes_api: Box::new(crate::apis::IssueVotesApiClient::new(rc.clone())),
            issue_watchers_api: Box::new(crate::apis::IssueWatchersApiClient::new(rc.clone())),
            issue_worklog_properties_api: Box::new(
                crate::apis::IssueWorklogPropertiesApiClient::new(rc.clone()),
            ),
            issue_worklogs_api: Box::new(crate::apis::IssueWorklogsApiClient::new(rc.clone())),
            issues_api: Box::new(crate::apis::IssuesApiClient::new(rc.clone())),
            jql_api: Box::new(crate::apis::JQLApiClient::new(rc.clone())),
            jira_expressions_api: Box::new(crate::apis::JiraExpressionsApiClient::new(rc.clone())),
            jira_settings_api: Box::new(crate::apis::JiraSettingsApiClient::new(rc.clone())),
            labels_api: Box::new(crate::apis::LabelsApiClient::new(rc.clone())),
            myself_api: Box::new(crate::apis::MyselfApiClient::new(rc.clone())),
            permission_schemes_api: Box::new(crate::apis::PermissionSchemesApiClient::new(
                rc.clone(),
            )),
            permissions_api: Box::new(crate::apis::PermissionsApiClient::new(rc.clone())),
            project_avatars_api: Box::new(crate::apis::ProjectAvatarsApiClient::new(rc.clone())),
            project_categories_api: Box::new(crate::apis::ProjectCategoriesApiClient::new(
                rc.clone(),
            )),
            project_components_api: Box::new(crate::apis::ProjectComponentsApiClient::new(
                rc.clone(),
            )),
            project_key_and_name_validation_api: Box::new(
                crate::apis::ProjectKeyAndNameValidationApiClient::new(rc.clone()),
            ),
            project_permission_schemes_api: Box::new(
                crate::apis::ProjectPermissionSchemesApiClient::new(rc.clone()),
            ),
            project_properties_api: Box::new(crate::apis::ProjectPropertiesApiClient::new(
                rc.clone(),
            )),
            project_role_actors_api: Box::new(crate::apis::ProjectRoleActorsApiClient::new(
                rc.clone(),
            )),
            project_roles_api: Box::new(crate::apis::ProjectRolesApiClient::new(rc.clone())),
            project_types_api: Box::new(crate::apis::ProjectTypesApiClient::new(rc.clone())),
            project_versions_api: Box::new(crate::apis::ProjectVersionsApiClient::new(rc.clone())),
            projects_api: Box::new(crate::apis::ProjectsApiClient::new(rc.clone())),
            screens_api: Box::new(crate::apis::ScreensApiClient::new(rc.clone())),
            server_info_api: Box::new(crate::apis::ServerInfoApiClient::new(rc.clone())),
            tasks_api: Box::new(crate::apis::TasksApiClient::new(rc.clone())),
            time_tracking_api: Box::new(crate::apis::TimeTrackingApiClient::new(rc.clone())),
            user_properties_api: Box::new(crate::apis::UserPropertiesApiClient::new(rc.clone())),
            user_search_api: Box::new(crate::apis::UserSearchApiClient::new(rc.clone())),
            users_api: Box::new(crate::apis::UsersApiClient::new(rc.clone())),
            webhooks_api: Box::new(crate::apis::WebhooksApiClient::new(rc.clone())),
            workflow_scheme_drafts_api: Box::new(crate::apis::WorkflowSchemeDraftsApiClient::new(
                rc.clone(),
            )),
            workflow_scheme_project_associations_api: Box::new(
                crate::apis::WorkflowSchemeProjectAssociationsApiClient::new(rc.clone()),
            ),
            workflow_schemes_api: Box::new(crate::apis::WorkflowSchemesApiClient::new(rc.clone())),
            workflow_status_categories_api: Box::new(
                crate::apis::WorkflowStatusCategoriesApiClient::new(rc.clone()),
            ),
            workflow_statuses_api: Box::new(crate::apis::WorkflowStatusesApiClient::new(
                rc.clone(),
            )),
            workflow_transition_properties_api: Box::new(
                crate::apis::WorkflowTransitionPropertiesApiClient::new(rc.clone()),
            ),
            workflow_transition_rules_api: Box::new(
                crate::apis::WorkflowTransitionRulesApiClient::new(rc.clone()),
            ),
            workflows_api: Box::new(crate::apis::WorkflowsApiClient::new(rc.clone())),
        }
    }

    pub fn app_properties_api(&self) -> &dyn crate::apis::AppPropertiesApi {
        self.app_properties_api.as_ref()
    }

    pub fn application_roles_api(&self) -> &dyn crate::apis::ApplicationRolesApi {
        self.application_roles_api.as_ref()
    }

    pub fn audit_records_api(&self) -> &dyn crate::apis::AuditRecordsApi {
        self.audit_records_api.as_ref()
    }

    pub fn avatars_api(&self) -> &dyn crate::apis::AvatarsApi {
        self.avatars_api.as_ref()
    }

    pub fn dashboards_api(&self) -> &dyn crate::apis::DashboardsApi {
        self.dashboards_api.as_ref()
    }

    pub fn dynamic_modules_api(&self) -> &dyn crate::apis::DynamicModulesApi {
        self.dynamic_modules_api.as_ref()
    }

    pub fn filter_sharing_api(&self) -> &dyn crate::apis::FilterSharingApi {
        self.filter_sharing_api.as_ref()
    }

    pub fn filters_api(&self) -> &dyn crate::apis::FiltersApi {
        self.filters_api.as_ref()
    }

    pub fn group_and_user_picker_api(&self) -> &dyn crate::apis::GroupAndUserPickerApi {
        self.group_and_user_picker_api.as_ref()
    }

    pub fn groups_api(&self) -> &dyn crate::apis::GroupsApi {
        self.groups_api.as_ref()
    }

    pub fn issue_attachments_api(&self) -> &dyn crate::apis::IssueAttachmentsApi {
        self.issue_attachments_api.as_ref()
    }

    pub fn issue_comment_properties_api(&self) -> &dyn crate::apis::IssueCommentPropertiesApi {
        self.issue_comment_properties_api.as_ref()
    }

    pub fn issue_comments_api(&self) -> &dyn crate::apis::IssueCommentsApi {
        self.issue_comments_api.as_ref()
    }

    pub fn issue_custom_field_options_api(&self) -> &dyn crate::apis::IssueCustomFieldOptionsApi {
        self.issue_custom_field_options_api.as_ref()
    }

    pub fn issue_custom_field_options_apps_api(
        &self,
    ) -> &dyn crate::apis::IssueCustomFieldOptionsAppsApi {
        self.issue_custom_field_options_apps_api.as_ref()
    }

    pub fn issue_field_configurations_api(&self) -> &dyn crate::apis::IssueFieldConfigurationsApi {
        self.issue_field_configurations_api.as_ref()
    }

    pub fn issue_fields_api(&self) -> &dyn crate::apis::IssueFieldsApi {
        self.issue_fields_api.as_ref()
    }

    pub fn issue_link_types_api(&self) -> &dyn crate::apis::IssueLinkTypesApi {
        self.issue_link_types_api.as_ref()
    }

    pub fn issue_links_api(&self) -> &dyn crate::apis::IssueLinksApi {
        self.issue_links_api.as_ref()
    }

    pub fn issue_navigator_settings_api(&self) -> &dyn crate::apis::IssueNavigatorSettingsApi {
        self.issue_navigator_settings_api.as_ref()
    }

    pub fn issue_notification_schemes_api(&self) -> &dyn crate::apis::IssueNotificationSchemesApi {
        self.issue_notification_schemes_api.as_ref()
    }

    pub fn issue_priorities_api(&self) -> &dyn crate::apis::IssuePrioritiesApi {
        self.issue_priorities_api.as_ref()
    }

    pub fn issue_properties_api(&self) -> &dyn crate::apis::IssuePropertiesApi {
        self.issue_properties_api.as_ref()
    }

    pub fn issue_remote_links_api(&self) -> &dyn crate::apis::IssueRemoteLinksApi {
        self.issue_remote_links_api.as_ref()
    }

    pub fn issue_resolutions_api(&self) -> &dyn crate::apis::IssueResolutionsApi {
        self.issue_resolutions_api.as_ref()
    }

    pub fn issue_search_api(&self) -> &dyn crate::apis::IssueSearchApi {
        self.issue_search_api.as_ref()
    }

    pub fn issue_security_level_api(&self) -> &dyn crate::apis::IssueSecurityLevelApi {
        self.issue_security_level_api.as_ref()
    }

    pub fn issue_security_schemes_api(&self) -> &dyn crate::apis::IssueSecuritySchemesApi {
        self.issue_security_schemes_api.as_ref()
    }

    pub fn issue_type_properties_api(&self) -> &dyn crate::apis::IssueTypePropertiesApi {
        self.issue_type_properties_api.as_ref()
    }

    pub fn issue_types_api(&self) -> &dyn crate::apis::IssueTypesApi {
        self.issue_types_api.as_ref()
    }

    pub fn issue_votes_api(&self) -> &dyn crate::apis::IssueVotesApi {
        self.issue_votes_api.as_ref()
    }

    pub fn issue_watchers_api(&self) -> &dyn crate::apis::IssueWatchersApi {
        self.issue_watchers_api.as_ref()
    }

    pub fn issue_worklog_properties_api(&self) -> &dyn crate::apis::IssueWorklogPropertiesApi {
        self.issue_worklog_properties_api.as_ref()
    }

    pub fn issue_worklogs_api(&self) -> &dyn crate::apis::IssueWorklogsApi {
        self.issue_worklogs_api.as_ref()
    }

    pub fn issues_api(&self) -> &dyn crate::apis::IssuesApi {
        self.issues_api.as_ref()
    }

    pub fn jql_api(&self) -> &dyn crate::apis::JQLApi {
        self.jql_api.as_ref()
    }

    pub fn jira_expressions_api(&self) -> &dyn crate::apis::JiraExpressionsApi {
        self.jira_expressions_api.as_ref()
    }

    pub fn jira_settings_api(&self) -> &dyn crate::apis::JiraSettingsApi {
        self.jira_settings_api.as_ref()
    }

    pub fn labels_api(&self) -> &dyn crate::apis::LabelsApi {
        self.labels_api.as_ref()
    }

    pub fn myself_api(&self) -> &dyn crate::apis::MyselfApi {
        self.myself_api.as_ref()
    }

    pub fn permission_schemes_api(&self) -> &dyn crate::apis::PermissionSchemesApi {
        self.permission_schemes_api.as_ref()
    }

    pub fn permissions_api(&self) -> &dyn crate::apis::PermissionsApi {
        self.permissions_api.as_ref()
    }

    pub fn project_avatars_api(&self) -> &dyn crate::apis::ProjectAvatarsApi {
        self.project_avatars_api.as_ref()
    }

    pub fn project_categories_api(&self) -> &dyn crate::apis::ProjectCategoriesApi {
        self.project_categories_api.as_ref()
    }

    pub fn project_components_api(&self) -> &dyn crate::apis::ProjectComponentsApi {
        self.project_components_api.as_ref()
    }

    pub fn project_key_and_name_validation_api(
        &self,
    ) -> &dyn crate::apis::ProjectKeyAndNameValidationApi {
        self.project_key_and_name_validation_api.as_ref()
    }

    pub fn project_permission_schemes_api(&self) -> &dyn crate::apis::ProjectPermissionSchemesApi {
        self.project_permission_schemes_api.as_ref()
    }

    pub fn project_properties_api(&self) -> &dyn crate::apis::ProjectPropertiesApi {
        self.project_properties_api.as_ref()
    }

    pub fn project_role_actors_api(&self) -> &dyn crate::apis::ProjectRoleActorsApi {
        self.project_role_actors_api.as_ref()
    }

    pub fn project_roles_api(&self) -> &dyn crate::apis::ProjectRolesApi {
        self.project_roles_api.as_ref()
    }

    pub fn project_types_api(&self) -> &dyn crate::apis::ProjectTypesApi {
        self.project_types_api.as_ref()
    }

    pub fn project_versions_api(&self) -> &dyn crate::apis::ProjectVersionsApi {
        self.project_versions_api.as_ref()
    }

    pub fn projects_api(&self) -> &dyn crate::apis::ProjectsApi {
        self.projects_api.as_ref()
    }

    pub fn screens_api(&self) -> &dyn crate::apis::ScreensApi {
        self.screens_api.as_ref()
    }

    pub fn server_info_api(&self) -> &dyn crate::apis::ServerInfoApi {
        self.server_info_api.as_ref()
    }

    pub fn tasks_api(&self) -> &dyn crate::apis::TasksApi {
        self.tasks_api.as_ref()
    }

    pub fn time_tracking_api(&self) -> &dyn crate::apis::TimeTrackingApi {
        self.time_tracking_api.as_ref()
    }

    pub fn user_properties_api(&self) -> &dyn crate::apis::UserPropertiesApi {
        self.user_properties_api.as_ref()
    }

    pub fn user_search_api(&self) -> &dyn crate::apis::UserSearchApi {
        self.user_search_api.as_ref()
    }

    pub fn users_api(&self) -> &dyn crate::apis::UsersApi {
        self.users_api.as_ref()
    }

    pub fn webhooks_api(&self) -> &dyn crate::apis::WebhooksApi {
        self.webhooks_api.as_ref()
    }

    pub fn workflow_scheme_drafts_api(&self) -> &dyn crate::apis::WorkflowSchemeDraftsApi {
        self.workflow_scheme_drafts_api.as_ref()
    }

    pub fn workflow_scheme_project_associations_api(
        &self,
    ) -> &dyn crate::apis::WorkflowSchemeProjectAssociationsApi {
        self.workflow_scheme_project_associations_api.as_ref()
    }

    pub fn workflow_schemes_api(&self) -> &dyn crate::apis::WorkflowSchemesApi {
        self.workflow_schemes_api.as_ref()
    }

    pub fn workflow_status_categories_api(&self) -> &dyn crate::apis::WorkflowStatusCategoriesApi {
        self.workflow_status_categories_api.as_ref()
    }

    pub fn workflow_statuses_api(&self) -> &dyn crate::apis::WorkflowStatusesApi {
        self.workflow_statuses_api.as_ref()
    }

    pub fn workflow_transition_properties_api(
        &self,
    ) -> &dyn crate::apis::WorkflowTransitionPropertiesApi {
        self.workflow_transition_properties_api.as_ref()
    }

    pub fn workflow_transition_rules_api(&self) -> &dyn crate::apis::WorkflowTransitionRulesApi {
        self.workflow_transition_rules_api.as_ref()
    }

    pub fn workflows_api(&self) -> &dyn crate::apis::WorkflowsApi {
        self.workflows_api.as_ref()
    }
}

mod tests {
    use crate::apis::configuration::Configuration;
    #[test]
    fn test_myself() {
        let mut secret =
            std::fs::read_to_string("./apikey.secret").expect("API Key secret file not found");
        secret.pop();
        let username = "spadavala@atlassian.com".to_string();

        let mut config = Configuration::default();
        let auth = (username, Some(secret));
        println!("{:?}", &auth);
        config.basic_auth = Some(auth);

        let client = super::APIClient::new(config);
        let myself = client.myself_api();
        let me = myself.get_current_user(None).unwrap();

        println!("{:?}", me);
    }
}
