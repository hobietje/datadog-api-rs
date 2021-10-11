use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::env;
use std::error;
use std::fmt;
use std::result;

// Datadog API HTTP Response body on errors
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DatadogErrorResponse {
  errors: Vec<String>
}

impl fmt::Display for DatadogErrorResponse {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.errors.join("\n"))
  }
}
impl error::Error for DatadogErrorResponse {}

// Wrapper for parsed Datadog API HTTP Response, whether success or error
pub type DatadogResult<T> = result::Result<T, Box<dyn error::Error>>;

// API Client
pub struct Client {
  pub(crate) host: String,
  pub(crate) api_key: String,
  pub(crate) application_key: String,
  pub(crate) client: reqwest::Client,
}

impl Default for Client {
  fn default() -> Client {
    let dd_host = env::var("DATADOG_HOST").unwrap_or("https://api.datadoghq.com".into());
    let dd_api_key = env::var("DD_API_KEY")
      .expect("Environment variable DD_API_KEY is needed to run the test suite");
    let dd_app_key = env::var("DD_APP_KEY")
      .expect("Environment variable DD_APP_KEY is needed to run the test suite");
    Client::new(&dd_host, &dd_api_key, &dd_app_key)
  }
}

impl Client {
  pub fn new(host: &str, api_key: &str, application_key: &str) -> Client {
    Client {
      host: host.into(),
      api_key: api_key.into(),
      application_key: application_key.into(),
      client: reqwest::Client::new(),
    }
  }

  pub async fn get_raw(
    &self,
    path_and_query: &str,
  ) -> result::Result<reqwest::Response, Box<dyn error::Error>> {
    let url = format!("{}{}", self.host, path_and_query);
    let res = self
      .client
      .get(url)
      .header("DD-API-KEY", self.api_key.to_string())
      .header("DD-APPLICATION-KEY", self.application_key.to_string())
      .send()
      .await?;
    Ok(res)
  }

  pub async fn get<'de, T: Serialize, R: DeserializeOwned>(
    &self,
    path_and_query: &str
  ) -> result::Result<R, Box<dyn error::Error>> {
    let resp = self.get_raw(path_and_query).await?;

    match &resp.status().is_success() {
      true => {
        let body = resp.text().await?;
        Ok(serde_json::from_str::<R>(
          &body,
        )?)
      }
      _ => {
        let body = &resp.text().await?;
        println!("{:?}", &body);
        Err(Box::new(serde_json::from_str::<DatadogErrorResponse>(
          &body,
        )?))
      }
    }
  }

  pub async fn post_jsonstr(
    &self,
    path_and_query: &str,
    json_str: &str,
  ) -> result::Result<reqwest::Response, Box<dyn error::Error>> {
    let url = format!("{}{}", self.host, path_and_query);
    let res = self
      .client
      .post(url)
      .header("DD-API-KEY", self.api_key.to_string())
      .header("DD-APPLICATION-KEY", self.application_key.to_string())
      .body(json_str.to_string())
      .send()
      .await?;
    Ok(res)
  }

  pub async fn post_json<T: Serialize>(
    &self,
    path_and_query: &str,
    json: &T,
  ) -> result::Result<reqwest::Response, Box<dyn error::Error>> {
    let url = format!("{}{}", self.host, path_and_query);
    let res = self
      .client
      .post(url)
      .header("DD-API-KEY", self.api_key.to_string())
      .header("DD-APPLICATION-KEY", self.application_key.to_string())
      .json(&json)
      .send()
      .await?;
    Ok(res)
  }

  pub async fn post<'de, T: Serialize, R: DeserializeOwned>(
    &self,
    path_and_query: &str,
    json: &T,
  ) -> result::Result<R, Box<dyn error::Error>> {
    let resp = self.post_json(path_and_query, json).await?;

    match &resp.status().is_success() {
      true => {
        let body = resp.text().await?;
        Ok(serde_json::from_str::<R>(
          &body,
        )?)
      }
      _ => {
        let body = &resp.text().await?;
        println!("{:?}", &body);
        Err(Box::new(serde_json::from_str::<DatadogErrorResponse>(
          &body,
        )?))
      }
    }
  }

  pub async fn post_str<'de, R: DeserializeOwned>(
    &self,
    path_and_query: &str,
    json_str: &str,
  ) -> result::Result<R, Box<dyn error::Error>> {
    let resp = self.post_jsonstr(path_and_query, json_str).await?;

    match &resp.status().is_success() {
      true => {
        let body = resp.text().await?;
        Ok(serde_json::from_str::<R>(
          &body,
        )?)
      }
      _ => {
        let body = &resp.text().await?;
        println!("{:?}", &body);
        Err(Box::new(serde_json::from_str::<DatadogErrorResponse>(
          &body,
        )?))
      }
    }
  }

  pub async fn put_jsonstr(
    &self,
    path_and_query: &str,
    json_str: &str,
  ) -> result::Result<reqwest::Response, Box<dyn error::Error>> {
    let url = format!("{}{}", self.host, path_and_query);
    let res = self
      .client
      .put(url)
      .header("DD-API-KEY", self.api_key.to_string())
      .header("DD-APPLICATION-KEY", self.application_key.to_string())
      .body(json_str.to_string())
      .send()
      .await?;
    Ok(res)
  }

  pub async fn put_json<T: Serialize>(
    &self,
    path_and_query: &str,
    json: &T,
  ) -> result::Result<reqwest::Response, Box<dyn error::Error>> {
    let url = format!("{}{}", self.host, path_and_query);
    let res = self
      .client
      .put(url)
      .header("DD-API-KEY", self.api_key.to_string())
      .header("DD-APPLICATION-KEY", self.application_key.to_string())
      .json(&json)
      .send()
      .await?;
    Ok(res)
  }

  pub async fn put<'de, T: Serialize, R: DeserializeOwned>(
    &self,
    path_and_query: &str,
    json: &T,
  ) -> result::Result<R, Box<dyn error::Error>> {
    let resp = self.put_json(path_and_query, json).await?;

    match &resp.status().is_success() {
      true => {
        let body = resp.text().await?;
        Ok(serde_json::from_str::<R>(
          &body,
        )?)
      }
      _ => {
        let body = &resp.text().await?;
        println!("{:?}", &body);
        Err(Box::new(serde_json::from_str::<DatadogErrorResponse>(
          &body,
        )?))
      }
    }
  }

  pub async fn put_str<'de, R: DeserializeOwned>(
    &self,
    path_and_query: &str,
    json_str: &str,
  ) -> result::Result<R, Box<dyn error::Error>> {
    let resp = self.put_jsonstr(path_and_query, json_str).await?;

    match &resp.status().is_success() {
      true => {
        let body = resp.text().await?;
        Ok(serde_json::from_str::<R>(
          &body,
        )?)
      }
      _ => {
        let body = &resp.text().await?;
        println!("{:?}", &body);
        Err(Box::new(serde_json::from_str::<DatadogErrorResponse>(
          &body,
        )?))
      }
    }
  }

  pub async fn delete_jsonstr(
    &self,
    path_and_query: &str,
    json_str: &str,
  ) -> result::Result<reqwest::Response, Box<dyn error::Error>> {
    let url = format!("{}{}", self.host, path_and_query);
    let res = self
      .client
      .delete(url)
      .header("DD-API-KEY", self.api_key.to_string())
      .header("DD-APPLICATION-KEY", self.application_key.to_string())
      .body(json_str.to_string())
      .send()
      .await?;
    Ok(res)
  }

  pub async fn delete_json<T: Serialize>(
    &self,
    path_and_query: &str,
    json: &T,
  ) -> result::Result<reqwest::Response, Box<dyn error::Error>> {
    let url = format!("{}{}", self.host, path_and_query);
    let res = self
      .client
      .delete(url)
      .header("DD-API-KEY", self.api_key.to_string())
      .header("DD-APPLICATION-KEY", self.application_key.to_string())
      .json(&json)
      .send()
      .await?;
    Ok(res)
  }

  pub async fn delete<'de, T: Serialize, R: DeserializeOwned>(
    &self,
    path_and_query: &str,
    json: &T,
  ) -> result::Result<R, Box<dyn error::Error>> {
    let resp = self.delete_json(path_and_query, json).await?;

    match &resp.status().is_success() {
      true => {
        let body = resp.text().await?;
        Ok(serde_json::from_str::<R>(
          &body,
        )?)
      }
      _ => {
        let body = &resp.text().await?;
        println!("{:?}", &body);
        Err(Box::new(serde_json::from_str::<DatadogErrorResponse>(
          &body,
        )?))
      }
    }
  }

  pub async fn delete_str<'de, R: DeserializeOwned>(
    &self,
    path_and_query: &str,
    json_str: &str,
  ) -> result::Result<R, Box<dyn error::Error>> {
    let resp = self.delete_jsonstr(path_and_query, json_str).await?;

    match &resp.status().is_success() {
      true => {
        let body = resp.text().await?;
        Ok(serde_json::from_str::<R>(
          &body,
        )?)
      }
      _ => {
        let body = &resp.text().await?;
        println!("{:?}", &body);
        Err(Box::new(serde_json::from_str::<DatadogErrorResponse>(
          &body,
        )?))
      }
    }
  }
}
