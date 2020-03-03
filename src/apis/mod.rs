use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod app_properties_api;
pub use self::app_properties_api::{AppPropertiesApi, AppPropertiesApiClient};
mod application_roles_api;
pub use self::application_roles_api::{ApplicationRolesApi, ApplicationRolesApiClient};
mod audit_records_api;
pub use self::audit_records_api::{AuditRecordsApi, AuditRecordsApiClient};
mod avatars_api;
pub use self::avatars_api::{AvatarsApi, AvatarsApiClient};
mod dashboards_api;
pub use self::dashboards_api::{DashboardsApi, DashboardsApiClient};
mod dynamic_modules_api;
pub use self::dynamic_modules_api::{DynamicModulesApi, DynamicModulesApiClient};
mod filter_sharing_api;
pub use self::filter_sharing_api::{FilterSharingApi, FilterSharingApiClient};
mod filters_api;
pub use self::filters_api::{FiltersApi, FiltersApiClient};
mod group_and_user_picker_api;
pub use self::group_and_user_picker_api::{GroupAndUserPickerApi, GroupAndUserPickerApiClient};
mod groups_api;
pub use self::groups_api::{GroupsApi, GroupsApiClient};
mod issue_attachments_api;
pub use self::issue_attachments_api::{IssueAttachmentsApi, IssueAttachmentsApiClient};
mod issue_comment_properties_api;
pub use self::issue_comment_properties_api::{
    IssueCommentPropertiesApi, IssueCommentPropertiesApiClient,
};
mod issue_comments_api;
pub use self::issue_comments_api::{IssueCommentsApi, IssueCommentsApiClient};
mod issue_custom_field_options_api;
pub use self::issue_custom_field_options_api::{
    IssueCustomFieldOptionsApi, IssueCustomFieldOptionsApiClient,
};
mod issue_custom_field_options_apps_api;
pub use self::issue_custom_field_options_apps_api::{
    IssueCustomFieldOptionsAppsApi, IssueCustomFieldOptionsAppsApiClient,
};
mod issue_field_configurations_api;
pub use self::issue_field_configurations_api::{
    IssueFieldConfigurationsApi, IssueFieldConfigurationsApiClient,
};
mod issue_fields_api;
pub use self::issue_fields_api::{IssueFieldsApi, IssueFieldsApiClient};
mod issue_link_types_api;
pub use self::issue_link_types_api::{IssueLinkTypesApi, IssueLinkTypesApiClient};
mod issue_links_api;
pub use self::issue_links_api::{IssueLinksApi, IssueLinksApiClient};
mod issue_navigator_settings_api;
pub use self::issue_navigator_settings_api::{
    IssueNavigatorSettingsApi, IssueNavigatorSettingsApiClient,
};
mod issue_notification_schemes_api;
pub use self::issue_notification_schemes_api::{
    IssueNotificationSchemesApi, IssueNotificationSchemesApiClient,
};
mod issue_priorities_api;
pub use self::issue_priorities_api::{IssuePrioritiesApi, IssuePrioritiesApiClient};
mod issue_properties_api;
pub use self::issue_properties_api::{IssuePropertiesApi, IssuePropertiesApiClient};
mod issue_remote_links_api;
pub use self::issue_remote_links_api::{IssueRemoteLinksApi, IssueRemoteLinksApiClient};
mod issue_resolutions_api;
pub use self::issue_resolutions_api::{IssueResolutionsApi, IssueResolutionsApiClient};
mod issue_search_api;
pub use self::issue_search_api::{IssueSearchApi, IssueSearchApiClient};
mod issue_security_level_api;
pub use self::issue_security_level_api::{IssueSecurityLevelApi, IssueSecurityLevelApiClient};
mod issue_security_schemes_api;
pub use self::issue_security_schemes_api::{
    IssueSecuritySchemesApi, IssueSecuritySchemesApiClient,
};
mod issue_type_properties_api;
pub use self::issue_type_properties_api::{IssueTypePropertiesApi, IssueTypePropertiesApiClient};
mod issue_types_api;
pub use self::issue_types_api::{IssueTypesApi, IssueTypesApiClient};
mod issue_votes_api;
pub use self::issue_votes_api::{IssueVotesApi, IssueVotesApiClient};
mod issue_watchers_api;
pub use self::issue_watchers_api::{IssueWatchersApi, IssueWatchersApiClient};
mod issue_worklog_properties_api;
pub use self::issue_worklog_properties_api::{
    IssueWorklogPropertiesApi, IssueWorklogPropertiesApiClient,
};
mod issue_worklogs_api;
pub use self::issue_worklogs_api::{IssueWorklogsApi, IssueWorklogsApiClient};
mod issues_api;
pub use self::issues_api::{IssuesApi, IssuesApiClient};
mod jql_api;
pub use self::jql_api::{JQLApi, JQLApiClient};
mod jira_expressions_api;
pub use self::jira_expressions_api::{JiraExpressionsApi, JiraExpressionsApiClient};
mod jira_settings_api;
pub use self::jira_settings_api::{JiraSettingsApi, JiraSettingsApiClient};
mod labels_api;
pub use self::labels_api::{LabelsApi, LabelsApiClient};
mod myself_api;
pub use self::myself_api::{MyselfApi, MyselfApiClient};
mod permission_schemes_api;
pub use self::permission_schemes_api::{PermissionSchemesApi, PermissionSchemesApiClient};
mod permissions_api;
pub use self::permissions_api::{PermissionsApi, PermissionsApiClient};
mod project_avatars_api;
pub use self::project_avatars_api::{ProjectAvatarsApi, ProjectAvatarsApiClient};
mod project_categories_api;
pub use self::project_categories_api::{ProjectCategoriesApi, ProjectCategoriesApiClient};
mod project_components_api;
pub use self::project_components_api::{ProjectComponentsApi, ProjectComponentsApiClient};
mod project_key_and_name_validation_api;
pub use self::project_key_and_name_validation_api::{
    ProjectKeyAndNameValidationApi, ProjectKeyAndNameValidationApiClient,
};
mod project_permission_schemes_api;
pub use self::project_permission_schemes_api::{
    ProjectPermissionSchemesApi, ProjectPermissionSchemesApiClient,
};
mod project_properties_api;
pub use self::project_properties_api::{ProjectPropertiesApi, ProjectPropertiesApiClient};
mod project_role_actors_api;
pub use self::project_role_actors_api::{ProjectRoleActorsApi, ProjectRoleActorsApiClient};
mod project_roles_api;
pub use self::project_roles_api::{ProjectRolesApi, ProjectRolesApiClient};
mod project_types_api;
pub use self::project_types_api::{ProjectTypesApi, ProjectTypesApiClient};
mod project_versions_api;
pub use self::project_versions_api::{ProjectVersionsApi, ProjectVersionsApiClient};
mod projects_api;
pub use self::projects_api::{ProjectsApi, ProjectsApiClient};
mod screens_api;
pub use self::screens_api::{ScreensApi, ScreensApiClient};
mod server_info_api;
pub use self::server_info_api::{ServerInfoApi, ServerInfoApiClient};
mod tasks_api;
pub use self::tasks_api::{TasksApi, TasksApiClient};
mod time_tracking_api;
pub use self::time_tracking_api::{TimeTrackingApi, TimeTrackingApiClient};
mod user_properties_api;
pub use self::user_properties_api::{UserPropertiesApi, UserPropertiesApiClient};
mod user_search_api;
pub use self::user_search_api::{UserSearchApi, UserSearchApiClient};
mod users_api;
pub use self::users_api::{UsersApi, UsersApiClient};
mod webhooks_api;
pub use self::webhooks_api::{WebhooksApi, WebhooksApiClient};
mod workflow_scheme_drafts_api;
pub use self::workflow_scheme_drafts_api::{
    WorkflowSchemeDraftsApi, WorkflowSchemeDraftsApiClient,
};
mod workflow_scheme_project_associations_api;
pub use self::workflow_scheme_project_associations_api::{
    WorkflowSchemeProjectAssociationsApi, WorkflowSchemeProjectAssociationsApiClient,
};
mod workflow_schemes_api;
pub use self::workflow_schemes_api::{WorkflowSchemesApi, WorkflowSchemesApiClient};
mod workflow_status_categories_api;
pub use self::workflow_status_categories_api::{
    WorkflowStatusCategoriesApi, WorkflowStatusCategoriesApiClient,
};
mod workflow_statuses_api;
pub use self::workflow_statuses_api::{WorkflowStatusesApi, WorkflowStatusesApiClient};
mod workflow_transition_properties_api;
pub use self::workflow_transition_properties_api::{
    WorkflowTransitionPropertiesApi, WorkflowTransitionPropertiesApiClient,
};
mod workflow_transition_rules_api;
pub use self::workflow_transition_rules_api::{
    WorkflowTransitionRulesApi, WorkflowTransitionRulesApiClient,
};
mod workflows_api;
pub use self::workflows_api::{WorkflowsApi, WorkflowsApiClient};

pub mod client;
pub mod configuration;
