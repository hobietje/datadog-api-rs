use datadog_api::Client;
use datadog_api::security_monitoring::*;
use tokio_test::block_on;

#[test]
fn default_list_rules() {
    let client = Client::default();
    let req = ListRulesRequest::default();
    let res = block_on(req.send(&client)).expect("API call failed");
    assert_eq!(10, res.data.len());
}

#[test]
fn paginated_list_rules() {
    let client = Client::default();
    let req = ListRulesRequest::default()
        .page_size(5)
        .page_number(2);
    let res = block_on(req.send(&client)).expect("API call failed");
    assert_eq!(5, res.data.len());
}
