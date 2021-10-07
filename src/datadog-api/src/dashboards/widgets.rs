// https://docs.datadoghq.com/dashboards/widgets/
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WidgetDefinition {
  #[serde(rename = "note")]
  Note(Note),
  #[serde(rename = "query_value")]
  QueryValue(QueryValue),
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

/// Aggregator used for the request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RequestAggregator {
  /// Average of all requests.
  #[serde(rename = "avg")]
  Average,
  /// Minimum of all requests.
  #[serde(rename = "min")]
  Minimum,
  /// Maximum of all requests.
  #[serde(rename = "max")]
  Maximum,
  /// Sum of all requests.
  #[serde(rename = "sum")]
  Sum,
  /// Last request value.
  #[serde(rename = "last")]
  Last,
  #[serde(rename = "percentile")]
  Percentile,
}
impl Default for RequestAggregator {
  fn default() -> RequestAggregator {
    RequestAggregator::Average
  }
}

/// The aggregation methods available for metrics queries.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetricAggregator {
  /// Average of all metrics values.
  #[serde(rename = "avg")]
  Average,
  /// Minimum of all metrics values.
  #[serde(rename = "min")]
  Minimum,
  /// Maximum of all metrics values.
  #[serde(rename = "max")]
  Maximum,
  /// Sum of all metrics values.
  #[serde(rename = "sum")]
  Sum,
  /// Last metrics value.
  #[serde(rename = "last")]
  Last,
  /// Signed area under the curve being graphed, which can be negative
  #[serde(rename = "area")]
  Area,
  /// Uses the norm of the timeseries, which is always positive, to rank the series.
  #[serde(rename = "l2norm")]
  Norm,
  #[serde(rename = "percentile")]
  Percentile,
}
impl Default for MetricAggregator {
  fn default() -> MetricAggregator {
    MetricAggregator::Average
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


/// List of custom links.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomLink {
  /// The flag for toggling context menu link visibility.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub is_hidden: Option<bool>,
  /// The label for the custom link URL. Keep the label short and descriptive. Use metrics and tags as variables.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<bool>,
  /// The URL of the custom link. URL must include `http` or `https`. A relative URL must start with `/`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub link: Option<bool>,
  /// The label ID that refers to a context menu link. Can be `logs`, `hosts`, `traces`, `profiles`, `processes`, `containers`, or `rum`.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub override_label: Option<bool>,
}

/// Time setting for the widget.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Time {
  /// The available timeframes depend on the widget you are using. Allowed enum values: `1m`,`5m`,`10m`,`15m`,`30m`,`1h`,`4h`,`1d`,`2d`,`1w`,`1mo`,`3mo`,`6mo`,`1y`,`alert`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub live_span: Option<String>, 
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Formula {
  /// Expression alias.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub alias: Option<String>,
  // pub cell_display_mode: String,
  // pub conditional_formats: String,
  /// String expression built from queries, formulas, and functions.
  pub formula: String,
  // pub limit:
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

/// Queries that can be returned directly or used in formulas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Query {
  Option1(QueryOption1),
}

/// A formula and functions metrics query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryOption1 {
  /// The aggregation methods available for metrics queries. Allowed enum values: `avg`,`min`,`max`,`sum`,`last`,`area`,`l2norm`,`percentile`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub aggregator: Option<MetricAggregator>,
  /// Data source for metrics queries. Allowed enum values: `metrics`
  pub data_source: String,
  /// Name of the query for use in formulas.
  pub name: String,
  /// Metrics query definition.
  pub query: String,
}

/// Timeseries or Scalar response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseFormat {
  #[serde(rename = "timeseries")]
  Timeseries,
  #[serde(rename = "scalar")]
  Scalar,
}

/// Widget definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Request {
  /// Aggregator used for the request. Allowed enum values: `avg`,`last`,`max`,`min`,`sum`,`percentile`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub aggregator: Option<RequestAggregator>,
  // apm_query
  // audit_query
  // conditional_formats
  // event_query
  /// List of formulas that operate on queries.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub formulas: Option<Vec<Formula>>,
  // log_query
  // network_query
  // process_query
  // profile_metrics_query
  // q
  /// List of queries that can be returned directly or used in formulas.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub queries: Option<Vec<Query>>,
  /// Timeseries or Scalar response. Allowed enum values: `timeseries`,`scalar`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub response_format: Option<ResponseFormat>
  // rum_query
  // security_query
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryValue {
  /// Whether to use auto-scaling or not.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub autoscale: Option<bool>,
  /// List of custom links.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub custom_links: Option<Vec<CustomLink>>,
  /// Display a unit of your choice on the widget.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub custom_unit: Option<String>,
  /// Number of decimals to show. If not defined, the widget uses the raw value.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub precision: Option<i64>,
  /// Widget definition.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub requests: Option<Vec<Request>>,
  /// How to align the text on the widget. Allowed enum values: `center`,`left`,`right`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text_align: Option<TextAlign>,
  /// Time setting for the widget.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub time: Option<Time>,
  /// Title of your widget.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub title: Option<String>,
  /// How to align the text on the widget. Allowed enum values: `center`,`left`,`right`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub title_align: Option<TextAlign>,
  /// Size of the title.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub title_size: Option<String>,
}