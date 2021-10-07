use datadog_api::Client;
use datadog_api::dashboards::*;
use tokio_test::block_on;

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
    let note = Note {
        content: "__Objective:__ Ensure classified data is adequately protected\n\n__Indicator:__ AWS S3 Bucket contents is effectively hidden from the broader Internet\n\n---\n\nExport CSV of [all assets](https://meetkunde.eng.roktinternal.com/aws/s3/bucket_effective_access_is_restricted_appropriately) | [non-compliant assets](https://meetkunde.eng.roktinternal.com/aws/s3/bucket_effective_access_is_restricted_appropriately?norm=Violated)".to_string(),
        background_color: Some("white".to_string()),
        font_size: Some("14".to_string()),
        text_align: Some(TextAlign::Left),
        vertical_align: Some(VerticalAlign::Top),
        show_tick: Some(false),
        tick_pos: Some("50%".to_string()),
        tick_edge: Some(TickEdge::Left),
        has_padding: Some(true),
    };
    let note_layout = WidgetLayout {
        is_column_break: None,
        x: 0,
        y: 0,
        width: 10,
        height: 2,
    };
    let note_widget = Widget {
        id: None,
        definition: Some(WidgetDefinition::Note(note)),
        layout: Some(note_layout),
    };
    let req = UpdateDashboardRequest::default()
        .dashboard_id("v2g-k58-bhe")
        .title("datadog-api-rs: test_update_dashboard_complete")
        .layout_type(LayoutType::Ordered)
        .widgets(vec!(
            note_widget
        ));
    let res = block_on(req.send(&client)).expect("API call failed");
    // assert_ne!(0, res.status);
}
