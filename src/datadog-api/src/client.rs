use serde::{Serialize, Deserialize};
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

  pub (crate) async fn get(&self, url: &str) -> result::Result<reqwest::Response, Box<dyn error::Error>> {
    let res = self.client.get(url)
                  .header("DD-API-KEY", self.api_key.to_string())
                  .header("DD-APPLICATION-KEY", self.application_key.to_string())
                  .send()
                  .await?;
    Ok(res)
  }

  pub (crate) async fn post<T:Serialize>(&self, url: &str, json: &T) -> result::Result<reqwest::Response, Box<dyn error::Error>> {
    let res = self.client.post(url)
                  .header("DD-API-KEY", self.api_key.to_string())
                  .header("DD-APPLICATION-KEY", self.application_key.to_string())
                  .json(&json)
                  .send()
                  .await?;
    Ok(res)
  }
}