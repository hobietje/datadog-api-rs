mod widgets;

pub use widgets::*;
use serde::{Serialize, Deserialize};
use crate::client::{*};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LayoutType {
  #[serde(rename = "ordered")]
  Ordered,
  #[serde(rename = "free")]
  Free,
}
impl Default for LayoutType {
  fn default() -> LayoutType {
    LayoutType::Ordered
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReflowType {
  #[serde(rename = "auto")]
  Auto,
  #[serde(rename = "fixed")]
  Fixed,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TemplateVariablePreset {}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TemplateVariable {}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WidgetLayout {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_column_break: Option<bool>,
  pub x: i64,
  pub y: i64,
  pub width: i64,
  pub height: i64,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Widget {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub id: Option<i64>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub definition: Option<WidgetDefinition>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub layout: Option<WidgetLayout>
}
impl Widget {
  pub fn definition(mut self, definition: WidgetDefinition) -> Widget {
    self.definition = Some(definition);
    self
  }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateDashboardRequest {
  // Query
  #[serde(skip_serializing)]
  pub dashboard_id: String,
  // Body
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_read_only: Option<bool>,
  pub layout_type: LayoutType,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub notify_list: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reflow_type: Option<ReflowType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub restricted_roles: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub template_variable_presets: Option<Vec<TemplateVariablePreset>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub template_variables: Option<Vec<TemplateVariable>>,
  pub title: String,
  pub widgets: Vec<Widget>,
}
impl UpdateDashboardRequest {
  pub fn dashboard_id(mut self, dashboard_id: &str) -> UpdateDashboardRequest {
    self.dashboard_id = dashboard_id.into();
    self
  }
  pub fn title(mut self, title: &str) -> UpdateDashboardRequest {
    self.title = title.into();
    self
  }
  pub fn layout_type(mut self, layout_type: LayoutType) -> UpdateDashboardRequest {
    self.layout_type = layout_type;
    self
  }
  pub fn widgets(mut self, widgets: Vec<Widget>) -> UpdateDashboardRequest {
    self.widgets = widgets;
    self
  }
}

impl UpdateDashboardRequest {
  pub async fn send(&self, client: &Client) -> DatadogResult<UpdateDashboardResponse> {
    let path_and_query = format!("/api/v1/dashboard/{}", self.dashboard_id);
    client.put::<UpdateDashboardRequest, UpdateDashboardResponse>(&path_and_query, &self).await
  }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateDashboardResponse {
  pub notify_list: Option<Vec<String>>,
  pub description: Option<String>,
  pub restricted_roles: Option<Vec<String>>,
  pub author_name: String,
  pub author_handle: String,
  pub template_variables: Option<Vec<TemplateVariable>>,
  pub is_read_only: bool,
  pub id: String,
  pub title: String,
  pub url: String,
  pub created_at: String,
  pub modified_at: String,
  pub layout_type: LayoutType,
  pub widgets: Vec<Widget>,
}

/// [Create a new dashboard](https://docs.datadoghq.com/api/latest/dashboards/#create-a-new-dashboard)
/// 
/// Create a dashboard using the specified options. When defining queries in your widgets, take note of which queries should have the as_count() or as_rate() modifiers appended. Refer to the following documentation for more information on these modifiers.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateDashboardRequest {
  /// Description of the dashboard.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  /// Whether this dashboard is read-only. If True, only the author and admins can make changes to it.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_read_only: Option<bool>,
  /// Layout type of the dashboard. Allowed enum values: `ordered`, `free`
  pub layout_type: LayoutType,
  /// List of handles of users to notify when changes are made to this dashboard.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub notify_list: Option<Vec<String>>,
  /// /// Reflow type for a new dashboard layout dashboard. Set this only when layout type is `ordered`. If set to `fixed`, the dashboard expects all widgets to have a layout, and if it's set to `auto`, widgets should not have layouts. Allowed enum values: `auto`, `fixed`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reflow_type: Option<ReflowType>,
  /// A list of role identifiers. Only the author and users associated with at least one of these roles can edit this dashboard. Overrides the `is_read_only` property if both are present
  #[serde(skip_serializing_if = "Option::is_none")]
  pub restricted_roles: Option<Vec<String>>,
  /// Array of template variables saved views.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub template_variable_presets: Option<Vec<TemplateVariablePreset>>,
  /// List of template variables for this dashboard.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub template_variables: Option<Vec<TemplateVariable>>,
  /// Title of the dashboard.
  pub title: String,
  /// List of widgets to display on the dashboard.
  pub widgets: Vec<Widget>,
}

impl CreateDashboardRequest {
  pub fn title(mut self, title: &str) -> CreateDashboardRequest {
    self.title = title.into();
    self
  }
  pub fn layout_type(mut self, layout_type: LayoutType) -> CreateDashboardRequest {
    self.layout_type = layout_type;
    self
  }
  pub fn widgets(mut self, widgets: Vec<Widget>) -> CreateDashboardRequest {
    self.widgets = widgets;
    self
  }

  pub async fn send(&self, client: &Client) -> DatadogResult<CreateDashboardResponse> {
    let path_and_query = "/api/v1/dashboard";
    client.post::<CreateDashboardRequest, CreateDashboardResponse>(&path_and_query, &self).await
  }
}

/// A dashboard is Datadog’s tool for visually tracking, analyzing, and displaying key performance metrics, which enable you to monitor the health of your infrastructure.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateDashboardResponse {
  /// Identifier of the dashboard author.
  pub author_handle: String,
  /// Creation date of the dashboard.
  pub created_at: String,
  /// Description of the dashboard.
  pub description: Option<String>,
  /// ID of the dashboard.
  pub id: String,
  /// Whether this dashboard is read-only. If True, only the author and admins can make changes to it.
  pub is_read_only: bool,
  /// Layout type of the dashboard. Allowed enum values: `ordered`, `free`
  pub layout_type: LayoutType,
  /// Modification date of the dashboard.
  pub modified_at: String,
  /// List of handles of users to notify when changes are made to this dashboard.
  pub notify_list: Option<Vec<String>>,
  /// Reflow type for a new dashboard layout dashboard. Set this only when layout type is `ordered`. If set to `fixed`, the dashboard expects all widgets to have a layout, and if it's set to `auto`, widgets should not have layouts. Allowed enum values: `auto`, `fixed`
  pub reflow_type: Option<ReflowType>,
  /// A list of role identifiers. Only the author and users associated with at least one of these roles can edit this dashboard. Overrides the `is_read_only` property if both are present.
  pub restricted_roles: Option<Vec<String>>,
  /// Array of template variables saved views.
  pub template_variable_presets: Option<Vec<TemplateVariablePreset>>,
  /// List of template variables for this dashboard.
  pub template_variables: Option<Vec<TemplateVariable>>,
  /// Title of the dashboard.
  pub title: String,
  /// The URL of the dashboard.
  pub url: String,
  /// List of widgets to display on the dashboard.
  pub widgets: Vec<Widget>,
}