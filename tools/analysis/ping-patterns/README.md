This directory contains a tool to analyze the patterns of baseline and metrics pings received on a per-client basis in Fenix.  These pattern analyses are then summarized in a single plot showing various patterns and issues found in the data.

## Requirements

A recent version of [`matplotlib`](https://matplotlib.org).

## Collecting input data

The input data is created using the following Redash query:

```sql
SELECT
  client_info.client_id,
  DATE(submission_timestamp) AS date,
  'metrics' as ping_type,
  client_info.app_display_version AS app_version,
  client_info.telemetry_sdk_build AS telemetry_sdk_build,
  client_info.android_sdk_version AS sdk,
  ping_info.start_time AS start_time,
  ping_info.end_time AS end_time,
  0 AS duration,
  ping_info.seq AS seq
FROM
  org_mozilla_fenix_nightly.metrics
WHERE
  DATE(submission_timestamp) > "2019-11-01"
UNION ALL
SELECT
  client_info.client_id,
  DATE(submission_timestamp) AS date,
  'baseline' as ping_type,
  client_info.app_display_version AS app_version,
  client_info.telemetry_sdk_build AS telemetry_sdk_build,
  client_info.android_sdk_version AS sdk,
  ping_info.start_time AS start_time,
  ping_info.end_time AS end_time,
  metrics.timespan.glean_baseline_duration.value AS duration,
  ping_info.seq AS seq
FROM
  org_mozilla_fenix_nightly.baseline
WHERE
  DATE(submission_timestamp) > "2019-11-01"
```

This query is also available [here](https://sql.telemetry.mozilla.org/queries/66682).

Save the result of the query as a `.csv` file to use as input to this script.

## Configuring the script

Configuration is performed by editing the `config.py` script.  There are comments as to what the fields do there.

## Running the script

To run the script, pass the input dataset and the output directory on the commandline.

```bash
$ ./ping-patterns.py dataset.csv plots
```

## Output

The output directory will contain one `.svg` file per client with a timeline of the baseline and metrics pings received from that client.

Baseline pings are in blue.  The thick part of the line represents the active session as returned in the `baseline.duration` metric. 

Metrics pings are in red.  Issues found with metrics pings are notated with a number to the right.  Hover over the number to display a tooltip with further information about the issue.

Gray vertical lines are midnight local time.  Dashed gray vertical lines are 04:00 local time.

Green vertical lines indicate the first ping coming from an interesting revision that contains a related fix.
