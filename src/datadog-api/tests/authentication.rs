use datadog_api::Client;
use datadog_api::authentication::*;
use tokio_test::block_on;

#[test]
fn validates_an_api_key() {
    let client = Client::default();
    let req = ValidateRequest {};
    let res = block_on(req.send(&client)).expect("API call failed");
    assert_eq!(true, res.valid);
}
