/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectInputBean {
    /// Project keys must be unique and start with an uppercase letter followed by one or more uppercase alphanumeric characters. The maximum length is 10 characters. Required when creating a project. Optional when updating a project.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the project. Required when creating a project. Optional when updating a project.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The [project type](https://confluence.atlassian.com/x/GwiiLQ#Jiraapplicationsoverview-Productfeaturesandprojecttypes), which dictates the application-specific feature set. Required when creating a project. Not applicable for the Update project resource.
    #[serde(rename = "projectTypeKey", skip_serializing_if = "Option::is_none")]
    pub project_type_key: Option<ProjectTypeKey>,
    /// A prebuilt configuration for a project. The type of the `projectTemplateKey` must match with the type of the `projectTypeKey`. Required when creating a project. Not applicable for the Update project resource.
    #[serde(rename = "projectTemplateKey", skip_serializing_if = "Option::is_none")]
    pub project_template_key: Option<ProjectTemplateKey>,
    /// A brief description of the project.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// This parameter is deprecated because of privacy changes. Use `leadAccountId` instead. See the [migration guide](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details. The user name of the project lead. Either `lead` or `leadAccountId` must be set when creating a project. Optional when updating a project. Cannot be provided with `leadAccountId`.
    #[serde(rename = "lead", skip_serializing_if = "Option::is_none")]
    pub lead: Option<String>,
    /// The account ID of the project lead. Either `lead` or `leadAccountId` must be set when creating a project. Optional when updating a project. Cannot be provided with `lead`.
    #[serde(rename = "leadAccountId", skip_serializing_if = "Option::is_none")]
    pub lead_account_id: Option<String>,
    /// A link to information about this project, such as project documentation
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The default assignee when creating issues for this project.
    #[serde(rename = "assigneeType", skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<AssigneeType>,
    /// An integer value for the project's avatar.
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    /// The ID of the issue security scheme for the project, which enables you to control who can and cannot view issues. Use the [Get issue security schemes](#api-rest-api-3-issuesecurityschemes-get) resource to get all issue security scheme IDs.
    #[serde(
        rename = "issueSecurityScheme",
        skip_serializing_if = "Option::is_none"
    )]
    pub issue_security_scheme: Option<i64>,
    /// The ID of the permission scheme for the project. Use the [Get all permission schemes](#api-rest-api-3-permissionscheme-get) resource to see a list of all permission scheme IDs.
    #[serde(rename = "permissionScheme", skip_serializing_if = "Option::is_none")]
    pub permission_scheme: Option<i64>,
    /// The ID of the notification scheme for the project. Use the [Get notification schemes](#api-rest-api-3-notificationscheme-get) resource to get a list of notification scheme IDs.
    #[serde(rename = "notificationScheme", skip_serializing_if = "Option::is_none")]
    pub notification_scheme: Option<i64>,
    /// The ID of the project's category. A complete list of category IDs is found using the [Get all project categories](#api-rest-api-3-projectCategory-get) operation.
    #[serde(rename = "categoryId", skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
}

impl ProjectInputBean {
    pub fn new() -> ProjectInputBean {
        ProjectInputBean {
            key: None,
            name: None,
            project_type_key: None,
            project_template_key: None,
            description: None,
            lead: None,
            lead_account_id: None,
            url: None,
            assignee_type: None,
            avatar_id: None,
            issue_security_scheme: None,
            permission_scheme: None,
            notification_scheme: None,
            category_id: None,
        }
    }
}

/// The [project type](https://confluence.atlassian.com/x/GwiiLQ#Jiraapplicationsoverview-Productfeaturesandprojecttypes), which dictates the application-specific feature set. Required when creating a project. Not applicable for the Update project resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectTypeKey {
    #[serde(rename = "software")]
    Software,
    #[serde(rename = "service_desk")]
    ServiceDesk,
    #[serde(rename = "business")]
    Business,
}
/// A prebuilt configuration for a project. The type of the `projectTemplateKey` must match with the type of the `projectTypeKey`. Required when creating a project. Not applicable for the Update project resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProjectTemplateKey {
    #[serde(rename = "com.pyxis.greenhopper.jira:gh-simplified-agility-kanban")]
    PyxisGreenhopperJiraghSimplifiedAgilityKanban,
    #[serde(rename = "com.pyxis.greenhopper.jira:gh-simplified-agility-scrum")]
    PyxisGreenhopperJiraghSimplifiedAgilityScrum,
    #[serde(rename = "com.pyxis.greenhopper.jira:gh-simplified-basic")]
    PyxisGreenhopperJiraghSimplifiedBasic,
    #[serde(rename = "com.pyxis.greenhopper.jira:gh-simplified-kanban-classic")]
    PyxisGreenhopperJiraghSimplifiedKanbanClassic,
    #[serde(rename = "com.pyxis.greenhopper.jira:gh-simplified-scrum-classic")]
    PyxisGreenhopperJiraghSimplifiedScrumClassic,
    #[serde(rename = "com.atlassian.servicedesk:simplified-it-service-desk")]
    AtlassianServicedesksimplifiedItServiceDesk,
    #[serde(rename = "com.atlassian.servicedesk:simplified-internal-service-desk")]
    AtlassianServicedesksimplifiedInternalServiceDesk,
    #[serde(rename = "com.atlassian.servicedesk:simplified-external-service-desk")]
    AtlassianServicedesksimplifiedExternalServiceDesk,
    #[serde(rename = "com.atlassian.servicedesk:simplified-hr-service-desk")]
    AtlassianServicedesksimplifiedHrServiceDesk,
    #[serde(rename = "com.atlassian.servicedesk:simplified-facilities-service-desk")]
    AtlassianServicedesksimplifiedFacilitiesServiceDesk,
    #[serde(
        rename = "com.atlassian.jira-core-project-templates:jira-core-simplified-content-management"
    )]
    AtlassianJiraCoreProjectTemplatesjiraCoreSimplifiedContentManagement,
    #[serde(
        rename = "com.atlassian.jira-core-project-templates:jira-core-simplified-document-approval"
    )]
    AtlassianJiraCoreProjectTemplatesjiraCoreSimplifiedDocumentApproval,
    #[serde(
        rename = "com.atlassian.jira-core-project-templates:jira-core-simplified-lead-tracking"
    )]
    AtlassianJiraCoreProjectTemplatesjiraCoreSimplifiedLeadTracking,
    #[serde(
        rename = "com.atlassian.jira-core-project-templates:jira-core-simplified-process-control"
    )]
    AtlassianJiraCoreProjectTemplatesjiraCoreSimplifiedProcessControl,
    #[serde(rename = "com.atlassian.jira-core-project-templates:jira-core-simplified-procurement")]
    AtlassianJiraCoreProjectTemplatesjiraCoreSimplifiedProcurement,
    #[serde(
        rename = "com.atlassian.jira-core-project-templates:jira-core-simplified-project-management"
    )]
    AtlassianJiraCoreProjectTemplatesjiraCoreSimplifiedProjectManagement,
    #[serde(rename = "com.atlassian.jira-core-project-templates:jira-core-simplified-recruitment")]
    AtlassianJiraCoreProjectTemplatesjiraCoreSimplifiedRecruitment,
    #[serde(rename = "com.atlassian.jira-core-project-templates:jira-core-simplified-task-")]
    AtlassianJiraCoreProjectTemplatesjiraCoreSimplifiedTask,
    #[serde(rename = "com.atlassian.jira.jira-incident-management-plugin:im-incident-management")]
    AtlassianJiraJiraIncidentManagementPluginimIncidentManagement,
}
/// The default assignee when creating issues for this project.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssigneeType {
    #[serde(rename = "PROJECT_LEAD")]
    PROJECTLEAD,
    #[serde(rename = "UNASSIGNED")]
    UNASSIGNED,
}
