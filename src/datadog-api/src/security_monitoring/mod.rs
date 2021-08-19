use serde::{Serialize, Deserialize};
use reqwest::Url;

use crate::client::{*};

/// List rules.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ListRulesRequest {
  /// Size for a given page.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub page_size: Option<i32>,
  /// Specific page number to return.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub page_number: Option<i32>,
}

impl ListRulesRequest {
  pub fn page_size(mut self, page_size: i32) -> ListRulesRequest {
    self.page_size = Some(page_size);
    self
  }
  pub fn page_number(mut self, page_number: i32) -> ListRulesRequest {
    self.page_number = Some(page_number);
    self
  }
  
  pub async fn send(&self, client: &Client) -> Result<ListRulesResponse> {
      let endpoint = format!("{}/api/v2/security_monitoring/rules", client.host);
      
      let mut url = Url::parse(&endpoint)?;
      if let Some(page_number) = &self.page_number {
        url.query_pairs_mut().append_pair("page[number]", &page_number.to_string());
      } 
      if let Some(page_size) = &self.page_size {
        url.query_pairs_mut().append_pair("page[size]", &page_size.to_string());
      } 
      println!("{}", url.as_str());

      
      let resp = client.get(url.as_str()).await?;

      match &resp.status().is_success() {
          true => {
              let body = &resp.text().await?;
              Ok(serde_json::from_str::<ListRulesResponse>(&body)?)
          },
          _ => {
              let body = &resp.text().await?;
              Err(Box::new(serde_json::from_str::<ErrorResponse>(&body)?))
          }
      }
  }
}

/// Severity of the Security Signal. Allowed enum values: `info`,`low`,`medium`,`high`,`critical`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
  #[serde(rename="info")]
  Info,
  #[serde(rename="low")]
  Low,
  #[serde(rename="medium")]
  Medium,
  #[serde(rename="high")]
  High,
  #[serde(rename="critical")]
  Critical,
}
impl Default for Status {
  fn default() -> Status {
    Status::Info
  }
}

/// Cases for generating signals.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Case {
  /// A rule case contains logical operations (`>`,`>=`, `&&`, `||`) to determine if a signal should be generated based on the event counts in the previously defined queries.
  pub condition: String,
  /// Name of the case.
  pub name: String,
  /// Notification targets for each rule case.
  pub notifications: Vec<String>,
  /// Severity of the Security Signal. Allowed enum values: `info`,`low`,`medium`,`high`,`critical`
  pub status: Status,
}

/// The type of filtering action. Allowed enum values: `require`,`suppress`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
  #[serde(rename="require")]
  Require,
  #[serde(rename="suppress")]
  Suppress,
}
impl Default for Action {
  fn default() -> Action {
    Action::Require
  }
}

/// Additional queries to filter matched events before they are processed.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Filter {
  /// The type of filtering action. Allowed enum values: `require`,`suppress`
  pub action: Action,
  /// Query for selecting logs to apply the filtering action.
  pub query: String,
}

/// Options on rules.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct NewValueOptions {
  /// The duration in days after which a learned value is forgotten. Allowed enum values: `1`,`2`,`7`,`14`,`21`,`28`
  #[serde(rename="forgetAfter")]
  pub forget_after: u8,
  /// The duration in days during which values are learned, and after which signals will be generated for values that weren't learned. If set to 0, a signal will be generated for all new values after the first value is learned. Allowed enum values: `0`,`1`,`7`
  #[serde(rename="learningDuration")]
  pub learning_duration: u8,
}

/// The detection method. Allowed enum values: `threshold`,`new_value`,`anomaly_detection`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DetectionMethod {
  #[serde(rename="threshold")]
  Threshold,
  #[serde(rename="new_value")]
  NewValue,
  #[serde(rename="anomaly_detection")]
  AnomalyDetection,
}
impl Default for DetectionMethod {
  fn default() -> DetectionMethod {
    DetectionMethod::Threshold
  }
}

/// Options on rules.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Options {
  /// The detection method. Allowed enum values: `threshold`,`new_value`,`anomaly_detection`
  #[serde(rename="detectionMethod")]
  pub detection_method: DetectionMethod,
  /// A time window is specified to match when at least one of the cases matches true. This is a sliding window and evaluates in real time. Allowed enum values: `0`,`60`,`300`,`600`,`900`,`1800`,`3600`,`7200`
  #[serde(rename="evaluationWindow")]
  pub evaluation_window: u16,
  /// Once a signal is generated, the signal will remain “open” if a case is matched at least once within this keep alive window. Allowed enum values: `0`,`60`,`300`,`600`,`900`,`1800`,`3600`,`7200`,`10800`,`21600`
  #[serde(rename="keepAlive")]
  pub keep_alive: u16,
  /// A signal will “close” regardless of the query being matched once the time exceeds the maximum duration. This time is calculated from the first seen timestamp. Allowed enum values: `0`,`60`,`300`,`600`,`900`,`1800`,`3600`,`7200`,`10800`,`21600`,`43200`,`86400`
  #[serde(rename="maxSignalDuration")]
  pub max_signal_duration: u32,
  #[serde(rename="newValueOptions")]
  pub new_value_options: NewValueOptions,
}

/// The Agent rule.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AgentRule {
  /// The Agent rule ID. Must be unique within the rule.
  #[serde(rename="agentRuleId")]
  pub agent_rule_id: Option<String>,
  /// A Runtime Security expression determines what activity should be collected by the Datadog Agent. These logical expressions can use predefined operators and attributes. Tags cannot be used in Runtime Security expressions. Instead, allow or deny based on tags under the advanced option.
  pub expression: Option<String>,
}

/// The aggregation type. Allowed enum values: `count`,`cardinality`,`sum`,`max`,`new_value`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Aggregation {
  #[serde(rename="count")]
  Count,
  #[serde(rename="cardinality")]
  Cardinality,
  #[serde(rename="sum")]
  Sum,
  #[serde(rename="max")]
  Max,
  #[serde(rename="new_value")]
  NewValue,
}
impl Default for Aggregation {
  fn default() -> Aggregation {
    Aggregation::Count
  }
}

/// Options on rules.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Query {
  /// The Agent rule.
  #[serde(rename="agentRule")]
  pub agent_rule: Option<AgentRule>,
  /// The aggregation type. Allowed enum values: `count`,`cardinality`,`sum`,`max`,`new_value`
  pub aggregation: Aggregation,
  /// Field for which the cardinality is measured. Sent as an array.
  #[serde(rename="distinctFields")]
  pub distinct_fields: Vec<String>,
  /// Fields to group by.
  #[serde(rename="groupByFields")]
  pub group_by_fields: Vec<String>,
  /// The target field to aggregate over when using the sum or max aggregations.
  pub metric: Option<String>,
  /// Name of the query.
  pub name: String,
  /// Query to run on logs.
  pub query: String,
}

/// Array containing the list of rules.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Data {
  /// Cases for generating signals.
  pub cases: Vec<Case>,
  /// When the rule was created, timestamp in milliseconds.
  #[serde(rename="createdAt")]
  pub created_at: i64,
  /// User ID of the user who created the rule.
  #[serde(rename="creationAuthorId")]
  pub creation_author_id: i64,
  /// Additional queries to filter matched events before they are processed.
  pub filters: Vec<Filter>,
  /// Whether the notifications include the triggering group-by values in their title.
  #[serde(rename="hasExtendedTitle")]
  pub has_extended_title: bool,
  /// The ID of the rule.
  pub id: String,
  /// Whether the rule is included by default.
  #[serde(rename="isDefault")]
  pub is_default: bool,
  /// Whether the rule has been deleted.
  #[serde(rename="isDeleted")]
  pub is_deleted: bool,
  /// Whether the rule is enabled.
  #[serde(rename="isEnabled")]
  pub is_enabled: bool,
  /// Message for generated signals.
  pub message: String,
  /// The name of the rule.
  pub name: String,
  /// Options on rules.
  pub options: Options,
  /// Queries for selecting logs which are part of the rule.
  pub queries: Vec<Query>,
  /// Tags for generated signals.
  pub tags: Vec<String>,
  /// User ID of the user who updated the rule.
  #[serde(rename="updateAuthorId")]
  pub update_author_id: i64,
  /// The version of the rule.
  pub version: i64,
}

/// Pagination object.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MetaPage {
  /// Total count.
  pub total_count: i64,
  /// Total count of elements matched by the filter.
  pub total_filtered_count: i64,
}

/// Object describing meta attributes of response.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Meta {
  /// Pagination object.
  pub page: Option<MetaPage>,
}

/// List of rules.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ListRulesResponse {
  /// Array containing the list of rules.
  pub data: Vec<Data>,
  /// Object describing meta attributes of response.
  pub meta: Meta,
}