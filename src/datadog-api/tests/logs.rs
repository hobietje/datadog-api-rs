use datadog_api::Client;
use datadog_api::logs::*;
use tokio_test::block_on;

#[test]
fn empty_search() {
    let client = Client::default();
    let req = SearchRequest::default();
    let res = block_on(req.send(&client)).expect("API call failed");
    assert_ne!(0, res.data.len());
}

#[test]
fn filtered_search() {
    let client = Client::default();
    let filter = Filter::default()
        .from("now-90d")
        .to("now")
        .indexes(vec!["*".to_string()])
        .query("hello");
    let options = Options::default()
        .timezone("UTC+10:00");
    let req = SearchRequest::default()
        .filter(filter)
        .options(options)
        .sort(Sort::TimestampDesc);
    let res = block_on(req.send(&client)).expect("API call failed");
    assert_ne!(0, res.data.len());
}

#[test]
fn paginated_search() {
    let client = Client::default();
    let page = Page::default()
        .limit(1);
    let req = SearchRequest::default()
        .page(page);
    let res = block_on(req.send(&client)).expect("API call failed");
    assert_ne!(0, res.data.len());

    if let Some(after) = res.meta.page {
        let next_page = Page::default()
            .limit(1)
            .cursor(&after.after);
        let next_req = SearchRequest::default()
            .page(next_page);
        let next_res = block_on(next_req.send(&client)).expect("API call failed");
        assert_ne!(0, next_res.data.len());
    } else {
        panic!("Only a single page of results was found")
    }
}
