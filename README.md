# Datadog API client [WIP]

Rust client for the Datadog API, using Tokio async.

# Todo

I will be implementing new functionality on an "as needed" basis only for projects I am working on.  I am not targeting 100% coverage of endpoints and features at this point in time.  Contributions are welcome!

* To evaluate macros for codegen similar API calls
* Handle rate limits
* Iterators or Streams for paginated result sets
* Paginated queries are ugly due to request builder being consumed
* Increase coverage of API endpoints

# API Coverage

| Done | Category                            | Endpoints   |
| ---- | ----------------------------------- | ----------- |
| Yes  | Authentication                      | GET /api/v1/validate |
| No   | AWS Integration                     | No          |
| No   | Azure Integration                   | No          |
| No   | Cloud Workload Security             | No          |
| No   | Dashboard Lists                     | No          |
| No   | Dashboards                          | No          |
| No   | Downtimes                           | No          |
| No   | Embeddable Graphs                   | No          |
| No   | Events                              | No          |
| No   | GCP Integration                     | No          |
| No   | Hosts                               | No          |
| No   | Incident Services                   | No          |
| No   | Incident Teams                      | No          |
| No   | Incidents                           | No          |
| No   | IP Ranges                           | No          |
| No   | Key Management                      | No          |
| 1/4  | Logs                                | POST /api/v2/logs/events/search |
| No   | Logs Archives                       | No          |
| No   | Logs Indexes                        | No          |
| No   | Logs Metrics                        | No          |
| No   | Logs Pipelines                      | No          |
| No   | Logs Restriction Queries            | No          |
| No   | Metrics                             | No          |
| No   | Monitors                            | No          |
| No   | Notebooks                           | No          |
| No   | Organizations                       | No          |
| No   | PagerDuty Integration               | No          |
| No   | Processes                           | No          |
| No   | Roles                               | No          |
| No   | Screenboards                        | No          |
| No   | Security Monitoring                 | No          |
| No   | Service Accounts                    | No          |
| No   | Service Checks                      | No          |
| No   | Service Dependencies                | No          |
| No   | Service Level Objective Corrections | No          |
| No   | Serivce Level Objectives            | No          |
| No   | Slack Integration                   | No          |
| No   | Snapshots                           | No          |
| No   | Synthetics                          | No          |
| No   | Tags                                | No          |
| No   | Timeboards                          | No          |
| No   | Tracing                             | No          |
| No   | Usage Metering                      | No          |
| No   | Users                               | No          |
| No   | Webhooks Integration                | No          |
