//! All requests to Datadog’s API must be authenticated. Requests that write data require reporting access and require an `API key`. Requests that read data require full access and also require an `application key`.
//!
//! __Note:__ All Datadog API clients are configured by default to consume Datadog US site APIs. If you are on the Datadog EU site, set the environment variable `DATADOG_HOST` to `https://api.datadoghq.eu` or override this value directly when creating your client.
//!
//! [Manage your account’s API and application keys](https://app.datadoghq.com/account/settings?_gl=1*1oolcm8*_ga*MTcyNzk1OTEzOC4xNTk3OTkxODkx*_ga_KN80RDFSQK*MTYyOTE3Mzk1Ny40NC4xLjE2MjkxNzU2MjYuMA..#api).

use serde::{Serialize, Deserialize};

use crate::client::{*};

/// Check if the API key (not the APP key) is valid. If invalid, a 403 is returned.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ValidateRequest {}
impl ValidateRequest {
  pub async fn send(&self, client: &Client) -> DatadogResult<ValidateResponse> {
    let path_and_query = "/api/v1/validate";
    client.get::<ValidateRequest, ValidateResponse>(path_and_query).await
}
}

/// Represent validation endpoint responses.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ValidateResponse {
  pub valid: bool
}
