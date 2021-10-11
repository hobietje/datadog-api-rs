use datadog_api::Client;
use datadog_api::dashboards::*;
use tokio_test::block_on;

/// Cretes a dashboard with the minimal amount of settings supported by DD
#[test]
fn test_create_dashboard_empty() {
    let client = Client::default();
    let req = CreateDashboardRequest::default()
        .title("datadog-api-rs: test_create_dashboard_empty")
        .layout_type(LayoutType::Ordered);
    let res = block_on(req.send(&client)).expect("API call failed");
    // assert_ne!(0, res.status);
}

/// Updates a dashboard with the minimal amount of settings supported by DD
#[test]
fn test_update_dashboard_empty() {
    let client = Client::default();
    let req = UpdateDashboardRequest::default()
        .dashboard_id("v2g-k58-bhe")
        .title("datadog-api-rs: test_update_dashboard_empty")
        .layout_type(LayoutType::Ordered);
    let res = block_on(req.send(&client)).expect("API call failed");
    // assert_ne!(0, res.status);
}

/// Updates a dashboard with the all settings supported by DD
#[test]
fn test_update_dashboard_complete() {
    let client = Client::default();
    let note = Widget {
        id: None,
        definition: Some(WidgetDefinition::Note(Note {
            content: "__Objective:__ Ensure classified data is adequately protected\n\n__Indicator:__ AWS S3 Bucket contents is effectively hidden from the broader Internet\n\n---\n\nExport CSV of [all assets](https://meetkunde.eng.roktinternal.com/aws/s3/bucket_effective_access_is_restricted_appropriately) | [non-compliant assets](https://meetkunde.eng.roktinternal.com/aws/s3/bucket_effective_access_is_restricted_appropriately?norm=Violated)".to_string(),
            background_color: Some("white".to_string()),
            font_size: Some("14".to_string()),
            text_align: Some(TextAlign::Left),
            vertical_align: Some(VerticalAlign::Top),
            show_tick: Some(false),
            tick_pos: Some("50%".to_string()),
            tick_edge: Some(TickEdge::Left),
            has_padding: Some(true),
        })),
        layout: Some(WidgetLayout {
            is_column_break: None,
            x: 0,
            y: 0,
            width: 10,
            height: 2,
        }),
    };
    let queryvalue = Widget {
        id: None,
        definition: Some(WidgetDefinition::QueryValue(QueryValue {
            custom_links: None,
            custom_unit: None,
            text_align: None,
            time: None,
            title: Some("Total".to_string()),
            title_size: Some("16".to_string()),
            title_align: Some(TextAlign::Left),
            requests: Some(vec!(
                Request {
                    aggregator: None,
                    formulas: Some(vec!(
                        Formula {
                            alias: None,
                            formula: "query1".to_string(),
                        }
                    )),
                    response_format: Some(ResponseFormat::Scalar),
                    queries: Some(vec!(
                        Query::Option1(QueryOption1 {
                            query: "sum:meetkunde.kci.aws.s3.bucket_effective_access_is_restricted_appropriately.gauge{$team}".to_string(),
                            data_source: "metrics".to_string(),
                            name: "query1".to_string(),
                            aggregator: Some(MetricAggregator::Last),
                        })
                    ))
                }
            )),
            autoscale: Some(false),
            precision: Some(0)
        })),
        layout: Some(WidgetLayout {
            is_column_break: None,
            x: 10,
            y: 0,
            width: 2,
            height: 1
        }),
    };
    let req = UpdateDashboardRequest::default()
        .dashboard_id("v2g-k58-bhe")
        .title("datadog-api-rs: test_update_dashboard_complete")
        .layout_type(LayoutType::Ordered)
        .widgets(vec!(
            note,
            queryvalue,
        ));
    let res = block_on(req.send(&client)).expect("API call failed");
    // assert_ne!(0, res.status);
}
