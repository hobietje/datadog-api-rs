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

impl UpdateDashboardRequest {
  // pub fn filter(mut self, filter: Filter) -> UpdateDashboardRequest {
  //   self.filter = Some(filter);
  //   self
  // }
  
  pub async fn send(&self, client: &Client) -> Result<UpdateDashboardResponse> {
    if self.dashboard_id.is_empty() {
      return Err(Box::new(InputValidationError::new("Missing dashboard_id request parameter")))
    }
    let url = format!("{}/api/v1/dashboard/{}", client.host, self.dashboard_id);

    // let json = serde_json::to_string(&self);
    // println!("{:?}", json);

    let resp = client.client.put(url)
                          .header("DD-API-KEY", client.api_key.to_string())
                          .header("DD-APPLICATION-KEY", client.application_key.to_string())
                          .json(&self)
                          .send().await?;

    match &resp.status().is_success() {
        true => {
            let body = &resp.text().await?;
            println!("{:?}", &body);
            Ok(serde_json::from_str::<UpdateDashboardResponse>(&body)?)
        },
        _ => {
            let body = &resp.text().await?;
            Err(Box::new(serde_json::from_str::<ApiErrorResponse>(&body)?))
        }
    }
  }
}