use datadog_api::Client;
use datadog_api::dashboard_lists::*;
use tokio_test::block_on;

#[test]
fn dashboard_list_get_items() {
    let client = Client::default();
    let req = GetDashboardListItemsRequest::default()
        .dashboard_list_id(242202);
    let res = block_on(req.send(&client)).expect("API call failed");
    assert_ne!(0, res.dashboards.len());
}

#[test]
fn dashboard_list_delete_items() {
    let client = Client::default();
    let req = DeleteDashboardListItemsRequest::default()
        .dashboard_list_id(242202)
        .dashboards(vec!(DashboardReference {
            id: "bkt-hmh-j8u".to_string(),
            _type: "custom_timeboard".to_string(),
        }));
    let res = block_on(req.send(&client)).expect("API call failed");
    assert_ne!(0, res.deleted_dashboards_from_list.len());
}

#[test]
fn dashboard_list_add_items() {
    let client = Client::default();
    let req = AddDashboardListItemsRequest::default()
        .dashboard_list_id(242202)
        .dashboards(vec!(DashboardReference {
            id: "bkt-hmh-j8u".to_string(),
            _type: "custom_timeboard".to_string(),
        }));
    let res = block_on(req.send(&client)).expect("API call failed");
    assert_ne!(0, res.added_dashboards_to_list.len());
}