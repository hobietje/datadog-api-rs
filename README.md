# Datadog API client [WIP]

Rust client for the Datadog API, using Tokio async.

# Usage

Initialize the client from environment variables `DATADOG_HOST`, `DD_API_KEY`, `DD_APP_KEY`:

```rs
let client = Client::default();
```
__Untyped Use:__

Create the HTTP Request body JSON string manually and send raw requests.

```rs
let json = "{}";
let path_and_query = format!("/api/v1/dashboard/{}", "das-hbo-ard");
let resp = client.put_jsonstr(&path_and_query, json).await.unwrap();
```

__Typed Use:__

Create the HTTP Request bodies using Rust Builders.  This will automatically parse HTTP Responses to typed objects as well.

```rs
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
let res =req.send(&client).await.unwrap();
```

# Todo

I will be implementing new functionality on an "as needed" basis only for projects I am working on.  I am not targeting 100% coverage of endpoints and features at this point in time.

* Macros to codegen similar parts of API calls
* Handle rate limits
* Iterators or Streams for paginated result sets
* Paginated queries are currently messy due to request builder being consumed, needs redesign?
* Increase coverage of API endpoints
* Utilities to handle date ranges, timezones, etc.

Contributions are welcome; send a PR.

## API Endpiont Coverage

Rust-style Builders are in place to build requests and parse responses for the following APIs:

| Done | Category                            |
| ---- | ----------------------------------- |
| Yes  | Authentication                      |
| No   | AWS Integration                     |
| No   | Azure Integration                   |
| No   | Cloud Workload Security             |
| 3/9  | Dashboard Lists                     |
| No   | Dashboards                          |
| No   | Downtimes                           |
| No   | Embeddable Graphs                   |
| No   | Events                              |
| No   | GCP Integration                     |
| No   | Hosts                               |
| No   | Incident Services                   |
| No   | Incident Teams                      |
| No   | Incidents                           |
| No   | IP Ranges                           |
| No   | Key Management                      |
| 1/4  | Logs                                |
| No   | Logs Archives                       |
| No   | Logs Indexes                        |
| No   | Logs Metrics                        |
| No   | Logs Pipelines                      |
| No   | Logs Restriction Queries            |
| No   | Metrics                             |
| No   | Monitors                            |
| No   | Notebooks                           |
| No   | Organizations                       |
| No   | PagerDuty Integration               |
| No   | Processes                           |
| No   | Roles                               |
| No   | Screenboards                        |
| No   | Security Monitoring                 |
| No   | Service Accounts                    |
| No   | Service Checks                      |
| No   | Service Dependencies                |
| No   | Service Level Objective Corrections |
| No   | Service Level Objectives            |
| No   | Slack Integration                   |
| No   | Snapshots                           |
| No   | Synthetics                          |
| No   | Tags                                |
| No   | Timeboards                          |
| No   | Tracing                             |
| No   | Usage Metering                      |
| No   | Users                               |
| No   | Webhooks Integration                |
