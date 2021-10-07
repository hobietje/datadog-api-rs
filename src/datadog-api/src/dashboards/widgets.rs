// https://docs.datadoghq.com/dashboards/widgets/
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WidgetDefinition {
  #[serde(rename = "note")]
  Note(Note),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WidgetType {
  #[serde(rename = "note")]
  Note,
}
impl Default for WidgetType {
  fn default() -> WidgetType {
    WidgetType::Note
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextAlign {
  #[serde(rename = "center")]
  Center,
  #[serde(rename = "left")]
  Left,
  #[serde(rename = "right")]
  Right,
}
impl Default for TextAlign {
  fn default() -> TextAlign {
    TextAlign::Left
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VerticalAlign {
  #[serde(rename = "top")]
  Top,
  #[serde(rename = "center")]
  Center,
  #[serde(rename = "bottom")]
  Bottom,
}
impl Default for VerticalAlign {
  fn default() -> VerticalAlign {
    VerticalAlign::Top
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TickEdge {
  #[serde(rename = "bottom")]
  Bottom,
  #[serde(rename = "left")]
  Left,
  #[serde(rename = "right")]
  Right,
  #[serde(rename = "top")]
  Top,
}
impl Default for TickEdge {
  fn default() -> TickEdge {
    TickEdge::Right
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub background_color: Option<String>,
  pub content: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub font_size: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub has_padding: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub show_tick: Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text_align: Option<TextAlign>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tick_edge: Option<TickEdge>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tick_pos: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub vertical_align: Option<VerticalAlign>,
}
impl Default for Note {
  fn default() -> Note {
    Note {
      background_color: None,
      content: "".to_string(),
      font_size: None,
      has_padding: None,
      show_tick: None,
      text_align: None,
      tick_edge: None,
      tick_pos: None,
      vertical_align: None,
    }
  }
}