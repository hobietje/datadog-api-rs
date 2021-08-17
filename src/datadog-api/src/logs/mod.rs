//! Search your logs and send them to your Datadog platform over HTTP.

use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::client::{*};

/// The minimum or maximum time for the requested logs.  
/// 
/// Supports date math and regular timestamps.
pub type Time = String;

/// Global query options that are used during the query. Note: You should only supply timezone or time offset but not both otherwise the query will fail.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Options {
  /// The time offset (in seconds) to apply to the query.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub time_offset: Option<u64>,
  /// The timezone can be specified both as an offset, for example: "UTC+03:00".
  #[serde(skip_serializing_if = "Option::is_none")]
  pub timezone: Option<String>,
}
impl Options {
  pub fn time_offset(mut self, time_offset: u64) -> Options {
    self.time_offset = Some(time_offset);
    self
  }
  pub fn timezone(mut self, timezone: &str) -> Options {
    self.timezone = Some(timezone.into());
    self
  }
}

/// Paging attributes for listing logs.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Page {
  /// List following results with a cursor provided in the previous query.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cursor: Option<String>,
  /// Maximum number of logs in the response.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub limit: Option<u32>,
}
impl Page {
  pub fn cursor(mut self, cursor: &str) -> Page {
    self.cursor = Some(cursor.into());
    self
  }
  pub fn limit(mut self, limit: u32) -> Page {
    self.limit = Some(limit);
    self
  }
}

/// Sort parameters when querying logs. 
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Sort {
  #[serde(rename="timestamp")]
  TimestampAsc,
  #[serde(rename="-timestamp")]
  TimestampDesc,
}

/// The search and filter query settings
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Filter {
  /// The minimum time for the requested logs, supports date math and regular timestamps
  #[serde(skip_serializing_if = "Option::is_none")]
  pub from: Option<Time>,
  /// For customers with multiple indexes, the indexes to search. Defaults to ['*'] which means all indexes.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub indexes: Option<Vec<String>>,
  /// The search query - following the log search syntax.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub query: Option<String>,
  /// The maximum time for the requested logs, supports date math and regular timestamps
  #[serde(skip_serializing_if = "Option::is_none")]
  pub to: Option<Time>,
}

impl Filter {
  pub fn from(mut self, from: &str) -> Filter {
    self.from = Some(from.into());
    self
  }
  pub fn to(mut self, to: &str) -> Filter {
    self.to = Some(to.into());
    self
  }
  pub fn indexes(mut self, indexes: Vec<String>) -> Filter {
    self.indexes = Some(indexes);
    self
  }
  pub fn query(mut self, query: &str) -> Filter {
    self.query = Some(query.into());
    self
  }
  pub fn build(self) -> Filter {
    self
  }
}

/// List endpoint returns logs that match a log search query. [Results are paginated](https://docs.datadoghq.com/logs/guide/collect-multiple-logs-with-pagination).
/// 
/// Use this endpoint to build complex logs filtering and search.
/// 
/// __If you are considering archiving logs for your organization, consider use of the Datadog archive capabilities instead of the log list API. See [Datadog Logs Archive documentation](https://docs.datadoghq.com/logs/archives).__
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchRequest {
  /// The search and filter query settings
  #[serde(skip_serializing_if = "Option::is_none")]
  pub filter: Option<Filter>,
  /// Global query options that are used during the query.
  /// 
  /// Note: You should only supply timezone or time offset but not both otherwise the query will fail.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub options: Option<Options>,
  /// Paging attributes for listing logs.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub page: Option<Page>,
  /// Sort parameters when querying logs. Allowed enum values: `timestamp`,`-timestamp`
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sort: Option<Sort>,
}

impl SearchRequest {
  pub fn filter(mut self, filter: Filter) -> SearchRequest {
    self.filter = Some(filter);
    self
  }
  pub fn options(mut self, options: Options) -> SearchRequest {
    self.options = Some(options);
    self
  }
  pub fn sort(mut self, sort: Sort) -> SearchRequest {
    self.sort = Some(sort);
    self
  }
  pub fn page(mut self, page: Page) -> SearchRequest {
    self.page = Some(page);
    self
  }
  
  pub async fn send(&self, client: &Client) -> Result<SearchResponse> {
      let url = format!("{}/api/v2/logs/events/search", client.host);

      let resp = client.client.post(url)
                            .header("DD-API-KEY", client.api_key.to_string())
                            .header("DD-APPLICATION-KEY", client.application_key.to_string())
                            .json(&self)
                            .send().await?;

      match &resp.status().is_success() {
          true => {
              let body = &resp.text().await?;
              Ok(serde_json::from_str::<SearchResponse>(&body)?)
          },
          _ => {
              let body = &resp.text().await?;
              Err(Box::new(serde_json::from_str::<ErrorResponse>(&body)?))
          }
      }
  }
}

/// JSON object containing all log attributes and their associated values.
pub type Attributes = HashMap<String, Value>;
fn default_attributes() -> HashMap<String, Value> { 
  HashMap::<String, Value>::default() 
}

/// Array of logs matching the request.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Data {
  /// JSON object containing all log attributes and their associated values.
  #[serde(default = "default_attributes")]
  pub attributes: Attributes,
  /// Unique ID of the Log.
  pub id: String,
  /// Type of the event. 
  /// 
  /// Allowed enum values: `log`
  #[serde(rename="type", skip_serializing_if = "Option::is_none")]
  pub data_type: Option<DataType>,
}



/// Type of an event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
  #[serde(rename="log")]
  Log,
}

/// The status of a response
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
  #[serde(rename="done")]
  Done,
  #[serde(rename="timeout")]
  Timeout,
}

/// A warning (non fatal error) encountered, partial results might be returned if warnings are present in a response.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Warning {
  /// A unique code for this type of warning
  pub code: String,
  /// A detailed explanation of this specific warning
  pub default: String,
  /// A short human-readable summary of the warning
  pub title: String,
}

/// Paging attributes.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MetaPage {
  /// The cursor to use to get the next results, if any. 
  /// 
  /// To make the next request, use the same parameters with the addition of the `page[cursor]`.
  pub after: String,
}

/// Links attributes.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Links {
  /// Link for the next set of results.
  pub next: String,
}

/// The metadata associated with a request
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Meta {
  /// The time elapsed in milliseconds
  pub elapsed: Option<u64>,
  /// Paging attributes.
  pub page: Option<MetaPage>,
  /// The identifier of the request
  pub request_id: Option<String>,
  /// The status of the response Allowed enum values: `done`,`timeout`
  pub status: Option<Status>,
  /// A list of warnings (non fatal errors) encountered, partial results might be returned if warnings are present in the response.
  pub warnings: Option<Vec<Warning>>
}

/// Response object with all logs matching the request and pagination information.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchResponse {
  /// Array of logs matching the request.
  pub data: Vec<Data>,
  /// Links attributes.
  pub links: Links,
  /// The metadata associated with a request
  pub meta: Meta,
}