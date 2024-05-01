//! Monitors allow you to watch a metric or check that you care about and notifies your team when a defined threshold has exceeded.
use crate::client::*;
use serde::{Deserialize, Serialize};
use async_gen::Return;
use std::{future::Future, pin::pin};
use std::collections::HashMap;
use async_gen::{self, AsyncGen, AsyncIter};
use futures_core::Stream;

/// Search and filter your monitors details.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MonitorsSearchRequest {
    /// After entering a search query in your [Manage Monitor page](https://app.datadoghq.com/monitors/manage?_gl=1*2jap8v*_ga*MTcyNzk1OTEzOC4xNTk3OTkxODkx*_ga_KN80RDFSQK*MTYzNzc5NTUxMi4xMDcuMS4xNjM3Nzk2NDAzLjA) use the query parameter value in the URL of the page as value for this parameter. Consult the dedicated [manage monitor documentation](https://docs.datadoghq.com/monitors/manage/#find-the-monitors) page to learn more.
    /// The query can contain any number of space-separated monitor attributes, for instance `query="type:metric status:alert"`.
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Page to start paginating from.
    pub page: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Number of monitors to return per page.
    pub per_page: Option<u64>,
    /// String for sort order, composed of field and sort order separate by a comma, e.g. `name,asc`. Supported sort directions: `asc`, `desc`. 
    /// Supported fields: `name`, `status`, `tags`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}
impl MonitorsSearchRequest {
    pub fn query(mut self, query: &str) -> MonitorsSearchRequest {
        self.query = query.into();
        self
    }
    pub fn page(mut self, page: u64) -> MonitorsSearchRequest {
        self.page = Some(page);
        self
    }
    pub fn per_page(mut self, per_page: u64) -> MonitorsSearchRequest {
        self.per_page = Some(per_page);
        self
    }
    pub fn sort(mut self, sort: &str) -> MonitorsSearchRequest {
        self.sort = Some(sort.into());
        self
    }
}

impl MonitorsSearchRequest {
    pub async fn send(&self, client: &Client) -> DatadogResult<MonitorsSearchResponse> {
        let mut queries: Vec<String> = vec![];
        queries.push(format!("query={}", &self.query));
        if let Some(page) = &self.page {
            queries.push(format!("page={}", &page));
        }
        if let Some(per_page) = &self.per_page {
            queries.push(format!("per_page={}", &per_page));
        }
        if let Some(sort) = &self.sort {
            queries.push(format!("sort={}", &sort));
        }
        let path = "/api/v1/monitor/search";
        let path_and_query = format!("{}?{}", &path, queries.join("&"));

        client
            .get::<MonitorsSearchRequest, MonitorsSearchResponse>(&path_and_query)
            .await
    }

    pub fn iter<'a>(&'a self, client: &'a Client) -> impl Stream<Item = Monitor> + 'a {
        let iter = AsyncIter::from(async_gen::gen! {
            let mut page = self.page.unwrap_or(0);
            loop {
                let request = MonitorsSearchRequest {
                    query: self.query.clone(),
                    page: Some(page),
                    per_page: self.per_page,
                    sort: self.sort.clone(),
                };
                let result = request.send(client).await.expect("Failed to call .send()");
                // Iterate over all results
                for item in result.monitors {
                    yield item;
                }
                // Stop iterating when we hit the last page
                if result.metadata.page >= result.metadata.page_count {
                    return;
                }
                page = page + 1;
            }
        });
        iter
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchFacetName {
    String(String),
    Bool(bool),
}
impl Default for SearchFacetName {
  fn default() -> SearchFacetName {
    SearchFacetName::String("".into())
  }
}

// Search facets.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchFacet {
    /// The facet value.
    pub name: SearchFacetName,
    /// The number of found monitors with the listed value.
    pub count: u64,
}

/// The counts of monitors per different criteria.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SearchFacetCounts {
    pub muted: Vec<SearchFacet>,
    pub status: Vec<SearchFacet>,
    pub tag: Vec<SearchFacet>,
    #[serde(rename = "type")]
    pub _type: Vec<SearchFacet>,
}

/// Metadata about the response.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Metadata {
    /// The page to start paginating from.
    pub page: u64,
    /// The number of pages.
    pub page_count: u64,
    /// The number of monitors to return per page.
    pub per_page: u64,
    /// The total number of monitors.
    pub total_count: u64,
}

/// Object describing the creator of the shared element.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct User {
    /// Email of the creator.
    pub email: String,
    /// Handle of the creator.
    pub handle: String,
    /// Name of the creator.
    pub name: String,
}

/// The notification triggered by the monitor.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Notification {
    /// The email address that received the notification.
    pub handle: String,
    /// The username receiving the notification
    pub name: String,
}

/// The different states your monitor can be in. Allowed enum values: `Alert`, `Ignored`, `No Data`, `OK`, `Skipped`, `Unknown`, `Warn`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Status {
    Alert,
    Ignored,
    #[serde(rename="No Data")]
    NoData,
    #[serde(rename="OK")]
    Ok,
    Skipped,
    Unknown,
    Warn
}
impl Default for Status {
  fn default() -> Status {
    Status::Unknown
  }
}

/// The type of the monitor. For more information about type, see the [monitor options](https://docs.datadoghq.com/monitors/guide/monitor_api_options/) docs.
/// Allowed enum values: `composite`, `event alert`, `log alert`, `metric alert`, `process alert`, `query alert`, `rum alert`, `service check`, `synthetics alert`, `trace-analytics alert`, `slo alert`, `event-v2 alert`, `audit alert`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MonitorType {
    #[serde(rename="composite")]
    Composite,
    #[serde(rename="event alert")]
    EventAlert,
    #[serde(rename="log alert")]
    LogAlert,
    #[serde(rename="metric alert")]
    MetricAlert,
    #[serde(rename="process alert")]
    ProcessAlert,
    #[serde(rename="query alert")]
    QueryAlert,
    #[serde(rename="rum alert")]
    RumAlert,
    #[serde(rename="service check")]
    ServiceCheck,
    #[serde(rename="synthetics alert")]
    SyntheticsAlert,
    #[serde(rename="trace-analytics aleert")]
    TraceAnalyticsAlert,
    #[serde(rename="slo alert")]
    SloAlert,
    #[serde(rename="event-v2 alert")]
    EventV2Alert,
    #[serde(rename="audit alert")]
    AuditAlert,
    #[serde(rename="ci-pipelines alert")]
    CiPipelinesAlert,
}
impl Default for MonitorType {
  fn default() -> MonitorType {
    MonitorType::LogAlert
  }
}

/// Datadog monitors
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Monitor {
    /// Classification of the monitor.
    pub classification: String,
    /// Object describing the creator of the shared element.
    pub creator: User,
    /// ID of the monitor.
    pub id: u64,
    /// Latest timestamp the monitor triggered.
    pub last_triggered_ts: Option<u64>,
    /// Metrics used by the monitor.
    pub metrics: Vec<String>,
    /// The monitor name.
    pub name: String,
    /// The notification triggered by the monitor.
    pub notifications: Vec<Notification>,
    /// The ID of the organization.
    pub org_id: u64,
    /// The scope(s) to which the downtime applies, e.g. `host:app2`. Provide multiple scopes as a comma-separated list, e.g. `env:dev,env:prod`. The resulting downtime applies to sources that matches ALL provided scopes (i.e. `env:dev AND env:prod`), NOT any of them.
    pub scopes: Vec<String>,
    /// The different states your monitor can be in. Allowed enum values: `Alert`, `Ignored`, `No Data`, `OK`, `Skipped`, `Unknown`, `Warn`
    pub status: Status,
    /// Tags associated with the monitor.
    pub tags: Vec<String>,
    /// The type of the monitor. For more information about type, see the [monitor options](https://docs.datadoghq.com/monitors/guide/monitor_api_options/) docs.
    /// Allowed enum values: `composite`, `event alert`, `log alert`, `metric alert`, `process alert`, `query alert`, `rum alert`, `service check`, `synthetics alert`, `trace-analytics alert`, `slo alert`, `event-v2 alert`, `audit alert`
    #[serde(rename="type")]
    pub _type: MonitorType,
}

/// The response form a monitor search.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MonitorsSearchResponse {
    /// The counts of monitors per different criteria.
    pub counts: SearchFacetCounts,
    /// Metadata about the response.
    pub metadata: Metadata,
    /// The list of found monitors.
    pub monitors: Vec<Monitor>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct MonitorOptions {
    /// Type of aggregation performed in the monitor query.
    pub aggregation: Aggregation,
    /// DEPRECATED: IDs of the device the Synthetics monitor is running on.
    pub device_ids: Vec<String>,
    /// Whether or not to send a log sample when the log monitor triggers.
    pub enable_logs_sample: bool,
    /// We recommend using the is_renotify, block in the original message instead. A message to include with a re-notification. Supports the @username notification we allow elsewhere. Not applicable if renotify_interval is None.
    pub escalation_message: String,
    /// Time (in seconds) to delay evaluation, as a non-negative integer. For example, if the value is set to 300 (5min), the timeframe is set to last_5m and the time is 7:00, the monitor evaluates data from 6:50 to 6:55. This is useful for AWS CloudWatch and other backfilled metrics to ensure the monitor always has data during evaluation.
    pub evaluation_delay: u64,
    /// Whether the log alert monitor triggers a single alert or multiple alerts when any group breaches a threshold.
    pub groupby_simple_monitor: bool,
    /// A Boolean indicating whether notifications from this monitor automatically inserts its triggering tags into the title.
    pub include_tags: bool,
    /// Whether or not the monitor is locked (only editable by creator and admins).
    pub locked: bool,
    /// How long the test should be in failure before alerting (integer, number of seconds, max 7200).
    pub min_failure_duration: u64,
    /// The minimum number of locations in failure at the same time during at least one moment in the min_failure_duration period (min_location_failed and min_failure_duration are part of the advanced alerting rules - integer, >= 1).
    pub min_location_failed: u64,
    /// Time (in seconds) to skip evaluations for new groups.
    /// For example, this option can be used to skip evaluations for new hosts while they initialize.
    /// Must be a non negative integer.
    pub new_group_delay: u64,
    /// DEPRECATED: Time (in seconds) to allow a host to boot and applications to fully start before starting the evaluation of monitor results. Should be a non negative integer.
    /// Use new_group_delay instead.
    pub new_host_delay: u64,
    /// The number of minutes before a monitor notifies after data stops reporting. Datadog recommends at least 2x the monitor timeframe for query alerts or 2 minutes for service checks. If omitted, 2x the evaluation timeframe is used for query alerts, and 24 hours is used for service checks.
    pub no_data_timeframe: u64,
    /// A Boolean indicating whether tagged users is notified on changes to this monitor.
    pub notify_audit: bool,
    /// A Boolean indicating whether this monitor notifies when data stops reporting.
    pub notify_no_data: bool,
    /// The number of minutes after the last notification before a monitor re-notifies on the current status. It only re-notifies if it’s not resolved.
    pub renotify_interval: u64,
    /// The number of times re-notification messages should be sent on the current status at the provided re-notification interval.
    pub renotify_occurrences: u64,
    /// The types of monitor statuses for which re-notification messages are sent.
    pub renotify_statuses: Vec<String>,
    /// A Boolean indicating whether this monitor needs a full window of data before it’s evaluated. We highly recommend you set this to false for sparse metrics, otherwise some evaluations are skipped. Default is false.
    pub require_full_window: bool,
    // DEPRECATED: Information about the downtime applied to the monitor.
    // pub silenced: object,
    /// DEPRECATED: ID of the corresponding Synthetic check.
    pub synthetics_check_id: String,
    /// Alerting time window options.
    pub threshold_windows: ThresholdWindows,
    /// List of the different monitor threshold available.
    pub thresholds: Thresholds,
    /// The number of hours of the monitor not reporting data before it automatically resolves from a triggered state.
    pub timeout_h: u64,
}


#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Aggregation {
    /// Group to break down the monitor on.
    pub group_by: String,
    /// Metric name used in the monitor.
    pub metric: String,
    /// Metric type used in the monitor.
    pub _type: String,
}

/// The different monitor threshold available.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Thresholds {
    /// The monitor `CRITICAL` threshold.
    pub critical: f64,
    /// The monitor CRITICAL recovery threshold.
    pub critical_recovery: f64,
    /// The monitor OK threshold.
    pub ok: f64,
    /// The monitor UNKNOWN threshold.
    pub unknown: f64,
    /// The monitor WARNING threshold.
    pub warning: f64,
    /// The monitor WARNING recovery threshold.
    pub warning_recovery: f64,
}

/// Alerting time window options.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ThresholdWindows {
    /// Describes how long an anomalous metric must be normal before the alert recovers.
    pub recovery_window: String,
    /// Describes how long a metric must be anomalous before an alert triggers.
    pub trigger_window: String,
}

/// Create a monitor request body.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateMonitorRequest {
    /// A message to include with notifications for this monitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The monitor name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of options associated with your monitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<MonitorOptions>,
    /// Integer from 1 (high) to 5 (low) indicating alert severity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
    /// The monitor query.
    pub query: String,
    /// A list of role identifiers that can be pulled from the Roles API. Cannot be used with `locked` option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_roles: Option<Vec<String>>,
    /// Tags associated to your monitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The type of the monitor. For more information about type, see the [monitor options](https://docs.datadoghq.com/monitors/guide/monitor_api_options/) docs.
    /// Allowed enum values: `composite`, `event alert`, `log alert`, `metric alert`, `process alert`, `query alert`, `rum alert`, `service check`, `synthetics alert`, `trace-analytics alert`, `slo alert`, `event-v2 alert`, `audit alert`
    #[serde(rename="type")]
    pub _type: MonitorType,
}
impl CreateMonitorRequest {
    pub fn message(mut self, message: &str) -> CreateMonitorRequest {
        self.message = Some(message.into());
        self
    }
    pub fn name(mut self, name: &str) -> CreateMonitorRequest {
        self.name = Some(name.into());
        self
    }
    pub fn options(mut self, options: MonitorOptions) -> CreateMonitorRequest {
        self.options = Some(options);
        self
    }
    pub fn priority(mut self, priority: u8) -> CreateMonitorRequest {
        self.priority = Some(priority);
        self
    }
    pub fn query(mut self, query: &str) -> CreateMonitorRequest {
        self.query = query.into();
        self
    }
    pub fn restricted_roles(mut self, restricted_roles: Vec<String>) -> CreateMonitorRequest {
        self.restricted_roles = Some(restricted_roles);
        self
    }
    pub fn tags(mut self, tags: Vec<String>) -> CreateMonitorRequest {
        self.tags = Some(tags);
        self
    }
    pub fn _type(mut self, _type: MonitorType) -> CreateMonitorRequest {
        self._type = _type.into();
        self
    }
}

impl CreateMonitorRequest {
    pub async fn send(&self, client: &Client) -> DatadogResult<CreateMonitorResponse> {
        let path_and_query = "/api/v1/monitor";

        client
            .post::<CreateMonitorRequest, CreateMonitorResponse>(&path_and_query, &self)
            .await
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CreateMonitorResponse {
    /// Timestamp of the monitor creation.
    pub created: String,
    /// Object describing the creator of the shared element.
    pub creator: User,
    /// Classification of the monitor.
    pub classification: String,
    /// Whether or not the monitor is deleted. (Always null)
    pub deleted: Option<String>,
    /// ID of the monitor.
    pub id: u64,
    /// A message to include with notifications for this monitor.
    pub message: String,
    /// Last timestamp when the monitor was edited.
    pub modified: String,
    /// Whether or not the monitor is broken down on different groups.
    pub multi: bool,
    /// The monitor name.
    pub name: String,
    /// List of options associated with your monitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<MonitorOptions>,
    // overall_state_modified?
    /// The different states your monitor can be in. Allowed enum values: `Alert`, `Ignored`, `No Data`, `OK`, `Skipped`, `Unknown`, `Warn`
    pub overall_state: Status,
    /// Integer from 1 (high) to 5 (low) indicating alert severity.
    pub priority: Option<u8>,
    /// The monitor query.
    pub query: String,
    /// A list of role identifiers that can be pulled from the Roles API. Cannot be used with `locked` option.
    pub restricted_roles: Option<Vec<String>>,
    /// Wrapper object with the different monitor states.
    pub state: State,
    /// Tags associated with the monitor.
    pub tags: Vec<String>,
    /// The type of the monitor. For more information about type, see the [monitor options](https://docs.datadoghq.com/monitors/guide/monitor_api_options/) docs.
    /// Allowed enum values: `composite`, `event alert`, `log alert`, `metric alert`, `process alert`, `query alert`, `rum alert`, `service check`, `synthetics alert`, `trace-analytics alert`, `slo alert`, `event-v2 alert`, `audit alert`
    #[serde(rename="type")]
    pub _type: MonitorType,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct State {
    /// Dictionary where the keys are groups (comma separated lists of tags) and the values are the list of groups your monitor is broken down on.
    pub groups: HashMap<String, GroupState>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct GroupState {
    /// Latest timestamp the monitor was in NO_DATA state.
    pub last_nodata_ts: u64,
    /// Latest timestamp of the notification sent for this monitor group.
    pub last_notified_ts: u64,
    /// Latest timestamp the monitor group was resolved.
    pub last_resolved_ts: u64,
    /// Latest timestamp the monitor group triggered.
    pub last_triggered_ts: u64,
    /// The name of the monitor.
    pub name: String,
    /// The different states your monitor can be in. Allowed enum values: Alert,Ignored,No Data,OK,Skipped,Unknown,Warn
    pub status: Status,
}


/// Edit the specified monitor.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EditMonitorRequest {
    // Path
    #[serde(skip_serializing)]
    pub monitor_id: u64,
    // Body
    /// A message to include with notifications for this monitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The monitor name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of options associated with your monitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<MonitorOptions>,
    /// Integer from 1 (high) to 5 (low) indicating alert severity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u8>,
    /// The monitor query.
    pub query: String,
    /// A list of role identifiers that can be pulled from the Roles API. Cannot be used with `locked` option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_roles: Option<Vec<String>>,
    /// Tags associated to your monitor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The type of the monitor. For more information about type, see the [monitor options](https://docs.datadoghq.com/monitors/guide/monitor_api_options/) docs.
    /// Allowed enum values: `composite`, `event alert`, `log alert`, `metric alert`, `process alert`, `query alert`, `rum alert`, `service check`, `synthetics alert`, `trace-analytics alert`, `slo alert`, `event-v2 alert`, `audit alert`
    #[serde(rename="type")]
    pub _type: MonitorType,
}
impl EditMonitorRequest {
    pub fn monitor_id(mut self, monitor_id: u64) -> EditMonitorRequest {
        self.monitor_id = monitor_id;
        self
    }
    pub fn message(mut self, message: &str) -> EditMonitorRequest {
        self.message = Some(message.into());
        self
    }
    pub fn name(mut self, name: &str) -> EditMonitorRequest {
        self.name = Some(name.into());
        self
    }
    pub fn options(mut self, options: MonitorOptions) -> EditMonitorRequest {
        self.options = Some(options);
        self
    }
    pub fn priority(mut self, priority: u8) -> EditMonitorRequest {
        self.priority = Some(priority);
        self
    }
    pub fn query(mut self, query: &str) -> EditMonitorRequest {
        self.query = query.into();
        self
    }
    pub fn restricted_roles(mut self, restricted_roles: Vec<String>) -> EditMonitorRequest {
        self.restricted_roles = Some(restricted_roles);
        self
    }
    pub fn tags(mut self, tags: Vec<String>) -> EditMonitorRequest {
        self.tags = Some(tags);
        self
    }
    pub fn _type(mut self, _type: MonitorType) -> EditMonitorRequest {
        self._type = _type.into();
        self
    }
}

impl EditMonitorRequest {
    pub async fn send(&self, client: &Client) -> DatadogResult<EditMonitorResponse> {
        let path_and_query = format!("/api/v1/monitor/{}", &self.monitor_id);

        client
            .put::<EditMonitorRequest, EditMonitorResponse>(&path_and_query, &self)
            .await
    }
}

/// Object describing a monitor
pub type EditMonitorResponse = CreateMonitorResponse;



/// Delete the specified monitor
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeleteMonitorRequest {
    // Path
    /// The ID of the monitor.
    #[serde(skip_serializing)]
    pub monitor_id: u64,
    /// Delete the monitor even if it’s referenced by other resources (e.g. SLO, composite monitor).
    #[serde(skip_serializing)]
    pub force: Option<bool>,
}
impl DeleteMonitorRequest {
    pub fn monitor_id(mut self, monitor_id: u64) -> DeleteMonitorRequest {
        self.monitor_id = monitor_id;
        self
    }
    pub fn force(mut self, force: bool) -> DeleteMonitorRequest {
        self.force = Some(force);
        self
    }
}

impl DeleteMonitorRequest {
    pub async fn send(&self, client: &Client) -> DatadogResult<DeleteMonitorResponse> {
        let mut queries: Vec<String> = vec![];
        if let Some(force) = &self.force {
            queries.push(format!("force={}", &force));
        }
        let path = format!("/api/v1/monitor/{}", &self.monitor_id);
        let path_and_query = match queries.len() {
          0 => path.to_string(),
          _ => format!("{}?{}", &path, queries.join("&")),
        };

        client
            .delete::<DeleteMonitorRequest, DeleteMonitorResponse>(&path_and_query, &self)
            .await
    }
}

/// Response from the delete monitor call.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeleteMonitorResponse {
    /// ID of the deleted monitor.
    pub deleted_monitor_id: u64,
}