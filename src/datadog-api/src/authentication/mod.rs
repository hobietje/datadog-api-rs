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
  pub async fn send(&self, client: &Client) -> Result<ValidateResponse> {
    let url = format!("{}/api/v1/validate", client.host);

    let resp = client.get(&url).await?;

    match &resp.status().is_success() {
        true => {
            let body = &resp.text().await?;
            Ok(serde_json::from_str::<ValidateResponse>(&body)?)
        },
        _ => {
            let body = &resp.text().await?;
            Err(Box::new(serde_json::from_str::<ErrorResponse>(&body)?))
        }
    }
}
}

/// Represent validation endpoint responses.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ValidateResponse {
  pub valid: bool
}
