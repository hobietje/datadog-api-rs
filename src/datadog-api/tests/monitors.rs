use std::pin::pin;

use datadog_api::Client;
use datadog_api::monitors::*;
use tokio_test::block_on;
use std::future::{poll_fn, Future};
use futures_util::StreamExt;

/// Minimal search for monitors
#[test]
fn test_monitors_search() {
    let client = Client::default();
    let req = MonitorsSearchRequest::default()
        // .query("tags(\"team:security\")");
        .query("security");
    let res = block_on(req.send(&client)).expect("API call failed");
    // assert_ne!(0, res.status);
}

/// Iterable search for monitors
#[test]
fn test_monitors_search_iter() {
    let res = block_on(async {
        let client = Client::default();
        let req = MonitorsSearchRequest::default()
            .query("security");
        let v: Vec<Monitor> = req.iter(&client).collect().await;
        println!("{}", v.len());
        assert_ne!(0, v.len());
    });
}

/// Creates a minimal monitor
#[test]
fn test_create_monitor() {
    let client = Client::default();
    let req = CreateMonitorRequest::default()
        .query("min(last_4h):sum:meetkunde.kci.aws.s3.bucket_effective_access_is_restricted_appropriately.gauge{is_compliant:true} / sum:meetkunde.kci.aws.s3.bucket_effective_access_is_restricted_appropriately.gauge{*} * 100 < 97")
        ._type(MonitorType::MetricAlert);
    let res = block_on(req.send(&client)).expect("API call failed");
    // assert_ne!(0, res.status);
}

/// Edits a minimal monitor
#[test]
fn test_edit_monitor() {
    let client = Client::default();
    let req = EditMonitorRequest::default()
        .monitor_id(55679649)
        .query("min(last_4h):sum:meetkunde.kci.aws.s3.bucket_effective_access_is_restricted_appropriately.gauge{is_compliant:true} / sum:meetkunde.kci.aws.s3.bucket_effective_access_is_restricted_appropriately.gauge{*} * 100 < 97")
        ._type(MonitorType::MetricAlert);
    let res = block_on(req.send(&client)).expect("API call failed");
    // assert_ne!(0, res.status);
}

/// Deletes a monitor
#[test]
fn test_delete_monitor() {
    let client = Client::default();
    let req = DeleteMonitorRequest::default()
        .monitor_id(55680057);
    let res = block_on(req.send(&client)).expect("API call failed");
    // assert_ne!(0, res.status);
}