use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::error;
use std::result;
use std::env;

// API Client
pub struct Client {
  pub (crate) host: String,
  pub (crate) api_key: String,
  pub (crate) application_key: String,
  pub (crate) client: reqwest::Client,
}

impl Default for Client {
  fn default() -> Client {
    let dd_host = env::var("DATADOG_HOST").unwrap_or("https://api.datadoghq.com".into());
    let dd_api_key = env::var("DD_API_KEY").expect("Environment variable DD_API_KEY is needed to run the test suite");
    let dd_app_key = env::var("DD_APP_KEY").expect("Environment variable DD_APP_KEY is needed to run the test suite");
    Client::new(&dd_host, &dd_api_key, &dd_app_key)
  }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ErrorResponse {
  errors: Vec<String>
}

impl fmt::Display for ErrorResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.errors.join("\n"))
  }
}
impl error::Error for ErrorResponse {}

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

impl Client {
  pub fn new(host: &str, api_key: &str, application_key: &str) -> Client {

      Client {
          host: host.into(),
          api_key: api_key.into(),
          application_key: application_key.into(),
          client: reqwest::Client::new()
      }
  }
}

// Logs
// https://docs.datadoghq.com/api/latest/logs/
pub type Time = String;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Filter {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub from: Option<Time>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub indexes: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub query: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub to: Option<Time>,
}
impl Filter {
  pub fn query(mut self, query: String) -> Filter {
    self.query = Some(query);
    self
  }
  pub fn build(self) -> Filter {
    self
  }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Options {
  pub time_offset: u64,
  pub timezone: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Page {
  pub cursor: String,
  pub limit: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Sort {
  #[serde(rename="timestamp")]
  TimestampAsc,
  #[serde(rename="-timestamp")]
  TimestampDesc,
}

/*
{
  "filter": {
    "from": "now-15m",
    "indexes": [
      "main",
      "web"
    ],
    "query": "service:web* AND @http.status_code:[200 TO 299]",
    "to": "now"
  },
  "options": {
    "timeOffset": "integer",
    "timezone": "GMT"
  },
  "page": {
    "cursor": "eyJzdGFydEF0IjoiQVFBQUFYS2tMS3pPbm40NGV3QUFBQUJCV0V0clRFdDZVbG8zY3pCRmNsbHJiVmxDWlEifQ==",
    "limit": 25
  },
  "sort": "string"
}
*/
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchRequest {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub filter: Option<Filter>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub options: Option<Options>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub page: Option<Page>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sort: Option<Sort>,
}

impl SearchRequest {
  pub fn filter(mut self, filter: Filter) -> SearchRequest {
    self.filter = Some(filter);
    self
  }
  
  pub async fn send(&self, client: &Client) -> Result<SearchResponse> {
      let url = format!("{}/api/v2/logs/events/search", client.host);

      // let json = serde_json::to_string(&self);
      // println!("{:?}", json);

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataType {
  #[serde(rename="log")]
  Log,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
  #[serde(rename="done")]
  Done,
  #[serde(rename="timeout")]
  Timeout,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Warning {
  pub code: String,
  pub default: String,
  pub title: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MetaPage {
  pub after: String,
}


pub type Attributes = HashMap<String, Value>;
fn default_attributes() -> HashMap<String, Value> { 
  HashMap::<String, Value>::default() 
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Data {
  #[serde(default = "default_attributes")]
  pub attributes: Attributes,
  pub id: String,
  #[serde(rename="type", skip_serializing_if = "Option::is_none")]
  pub data_type: Option<DataType>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Links {
  pub next: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Meta {
  // pub elapsed: u64,
  pub page: MetaPage,
  // pub request_id: String,
  // pub status: Option<Status>,
  // pub warnings: Vec<Warning>
}
/*
{
  "data": [
    {
      "attributes": {
        "attributes": [
          {}
        ],
        "host": "i-0123",
        "message": "Host connected to remote",
        "service": "agent",
        "status": "INFO",
        "tags": [
          "team:A"
        ],
        "timestamp": "2019-01-02T09:42:36.320Z"
      },
      "id": "AAAAAWgN8Xwgr1vKDQAAAABBV2dOOFh3ZzZobm1mWXJFYTR0OA",
      "type": "log"
    }
  ],
  "links": {
    "next": "https://app.datadoghq.com/api/v2/logs/event?filter[query]=foo\u0026page[cursor]=eyJzdGFydEF0IjoiQVFBQUFYS2tMS3pPbm40NGV3QUFBQUJCV0V0clRFdDZVbG8zY3pCRmNsbHJiVmxDWlEifQ=="
  },
  "meta": {
    "elapsed": 132,
    "page": {
      "after": "eyJzdGFydEF0IjoiQVFBQUFYS2tMS3pPbm40NGV3QUFBQUJCV0V0clRFdDZVbG8zY3pCRmNsbHJiVmxDWlEifQ=="
    },
    "request_id": "MWlFUjVaWGZTTTZPYzM0VXp1OXU2d3xLSVpEMjZKQ0VKUTI0dEYtM3RSOFVR",
    "status": "done",
    "warnings": [
      {
        "code": "unknown_index",
        "detail": "indexes: foo, bar",
        "title": "One or several indexes are missing or invalid, results hold data from the other indexes"
      }
    ]
  }
}
*/
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchResponse {
  pub data: Vec<Data>,
  pub links: Links,
  pub meta: Meta,
}