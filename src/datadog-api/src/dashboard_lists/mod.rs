//! Interact with your dashboard lists through the API to organize, find, and share all of your dashboards with your team and organization.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::client::*;

/// Creator of the object
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Author {
  /// Email of the creator
  pub email: Option<String>,
  /// Handle of the creator
  pub handle: String,
  /// Name of the creator
  pub name: String,
}
/// Dashboard within a list
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Dashboard {
  /// Creator of the object
  pub author: Author,
  /// Date of creation of the dashboard
  pub created: String,
  /// URL to the icon of the dashboard
  pub icon: Option<String>,
  /// ID of the dashboard
  pub id: String,
  /// Whether or not the dashboard is in the favorites
  pub is_favorite: bool,
  /// Whether or not the dashboard is read only
  pub is_read_only: bool,
  /// Whether the dashboard is publicly shared or not
  pub is_shared: bool,
  /// Date of last edition of the dashboard
  pub modified: String,
  /// Popularity of the dashboard
  pub popularity: i32,
  /// Title of the dashboard
  pub title: String,
  /// The type of the dashboard. Allowed enum values: `custom_timeboard`, `custom_screenboard`, `integration_screenboard`, `integration_timeboard`, `host_timeboard`
  #[serde(rename = "type")]
  pub _type: String,
  /// URL path to the dashboard
  pub url: String,
}

/// [Get items of a Dashboard List](https://docs.datadoghq.com/api/latest/dashboard-lists/#get-items-of-a-dashboard-list)
///
/// Fetch the dashboard list’s dashboard definitions.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetDashboardListItemsRequest {
  /// ID of the dashboard list to get items from
  #[serde(skip_serializing)]
  pub dashboard_list_id: i64,
}

impl GetDashboardListItemsRequest {
  pub fn dashboard_list_id(mut self, dashboard_list_id: i64) -> GetDashboardListItemsRequest {
    self.dashboard_list_id = dashboard_list_id;
    self
  }
  pub async fn send(&self, client: &Client) -> DatadogResult<GetDashboardListItemsResponse> {
    let path_and_query = format!(
      "/api/v2/dashboard/lists/manual/{dashboard_list_id}/dashboards",
      dashboard_list_id = self.dashboard_list_id
    );
    let resp = client.get(&path_and_query).await?;

    match &resp.status().is_success() {
      true => {
        let body = &resp.text().await?;
        Ok(serde_json::from_str::<GetDashboardListItemsResponse>(
          &body,
        )?)
      }
      _ => {
        let body = &resp.text().await?;
        Err(Box::new(serde_json::from_str::<DatadogErrorResponse>(
          &body,
        )?))
      }
    }
  }
}

/// Dashboards within a list.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GetDashboardListItemsResponse {
  /// List of dashboards in the dashboard list
  pub dashboards: Vec<Dashboard>,
  /// Number of dashboards in the dashboard list
  pub total: u64,
}

/// Dashboards to add the dashboard list
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DashboardReference {
  /// ID of the dashboard.
  pub id: String,
  /// The type of the dashboard. Allowed enum values: `custom_timeboard`, `custom_screenboard`, `integration_screenboard`, `integration_timeboard`, `host_timeboard`
  #[serde(rename = "type")]
  pub _type: String,
}

/// [Add Items to a Dashboard List](https://docs.datadoghq.com/api/latest/dashboard-lists/#add-items-to-a-dashboard-list)
///
/// Add dashboards to an existing dashboard list.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AddDashboardListItemsRequest {
  /// ID of the dashboard list to get items from
  #[serde(skip_serializing)]
  pub dashboard_list_id: i64,
  /// List of dashboards to add the dashboard list
  pub dashboards: Vec<DashboardReference>,
}

impl AddDashboardListItemsRequest {
  pub fn dashboard_list_id(mut self, dashboard_list_id: i64) -> AddDashboardListItemsRequest {
    self.dashboard_list_id = dashboard_list_id;
    self
  }
  pub fn dashboards(mut self, dashboards: Vec<DashboardReference>) -> AddDashboardListItemsRequest {
    self.dashboards = dashboards;
    self
  }
  pub async fn send(&self, client: &Client) -> DatadogResult<AddDashboardListItemsResponse> {
    let path_and_query = format!(
      "/api/v2/dashboard/lists/manual/{dashboard_list_id}/dashboards",
      dashboard_list_id = self.dashboard_list_id
    );
    let resp = client.post_json(&path_and_query, self).await?;

    match &resp.status().is_success() {
      true => {
        let body = &resp.text().await?;
        Ok(serde_json::from_str::<AddDashboardListItemsResponse>(
          &body,
        )?)
      }
      _ => {
        let body = &resp.text().await?;
        Err(Box::new(serde_json::from_str::<DatadogErrorResponse>(
          &body,
        )?))
      }
    }
  }
}

/// Response containing a list of added dashboards
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AddDashboardListItemsResponse {
  /// List of dashboards added to the dashboard list.
  pub added_dashboards_to_list: Vec<DashboardReference>,
}

/// [Delete items from a dashboard list](https://docs.datadoghq.com/api/latest/dashboard-lists/#delete-items-from-a-dashboard-list)
///
/// Delete dashboards from an existing dashboard list.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeleteDashboardListItemsRequest {
  /// ID of the dashboard list to delete items from
  #[serde(skip_serializing)]
  pub dashboard_list_id: i64,
  /// List of dashboards to delete from the dashboard list
  pub dashboards: Vec<DashboardReference>,
}

impl DeleteDashboardListItemsRequest {
  pub fn dashboard_list_id(mut self, dashboard_list_id: i64) -> DeleteDashboardListItemsRequest {
    self.dashboard_list_id = dashboard_list_id;
    self
  }
  pub fn dashboards(mut self, dashboards: Vec<DashboardReference>) -> DeleteDashboardListItemsRequest {
    self.dashboards = dashboards;
    self
  }
  pub async fn send(&self, client: &Client) -> DatadogResult<DeleteDashboardListItemsResponse> {
    let path_and_query = format!(
      "/api/v2/dashboard/lists/manual/{dashboard_list_id}/dashboards",
      dashboard_list_id = self.dashboard_list_id
    );
    let resp = client.delete_json(&path_and_query, self).await?;

    match &resp.status().is_success() {
      true => {
        let body = &resp.text().await?;
        Ok(serde_json::from_str::<DeleteDashboardListItemsResponse>(
          &body,
        )?)
      }
      _ => {
        let body = &resp.text().await?;
        Err(Box::new(serde_json::from_str::<DatadogErrorResponse>(
          &body,
        )?))
      }
    }
  }
}

/// Response containing a list of deleted dashboards
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeleteDashboardListItemsResponse {
  /// List of dashboards deleted from the dashboard list
  pub deleted_dashboards_from_list: Vec<DashboardReference>,
}