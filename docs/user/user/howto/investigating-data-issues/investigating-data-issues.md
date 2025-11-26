# Telemetry/Data Bug Investigation Recommendations

This document outlines several diagnostic categories and the insights they may offer when investigating unusual telemetry patterns or data anomalies.

### 1\. Countries

* Purpose: Identify geographical patterns that could explain anomalies.
* Column Name: `metadata.geo.country`
* Considerations:
  * Are there ongoing national holidays or similar events that could affect data?
  * Is the region known for bot activity or unusual behavior?

### 2\. ISP (Internet Service Provider)

* Purpose: Analyze data at a more granular level than countries to identify potential automation or bot activity.
* Column Name: `metadata.isp.name`
* Considerations:
  * Could the anomaly be traced back to a single ISP, potentially indicating automation?
  * Be mindful of the large number of ISPs; consider applying filters (e.g., `HAVING` clause) to exclude smaller ISPs.

### 3\. Product Version / Build ID

* Purpose: Check if issues began with a specific product version or build.
* Column Names: `client_info.app_display_version`, `client_info.app_build`
* Considerations:
  * Did the issue arise after a particular version update? If so, collaborate with the product team to identify changes.
  * Ensure that the build ID matches a known Mozilla build. If not, it could be a clone, fork, or side-load build.

### 4\. Glean SDK Version

* Purpose: Determine whether the issue is tied to a specific Glean SDK version.
* Column Name: `client_info.telemetry_sdk_build`
* Considerations:
  * Did the anomaly start after an update to Glean? Work with the Glean team to verify version changes.

### 5\. Other Library Version Changes

* Purpose: Identify possible regressions due to library updates.
* Considerations:
  * Review updates to Application Services, Gecko, and other dependencies (e.g., Viaduct, rkv) that could affect telemetry collection.

### 6\. OS/Platform SDK Version

* Purpose: Check if Operating System or platform SDK changes are impacting data collection.
* Column Names: `client_info.os_version` (Android only: `client_info.android_sdk_version`)
* Considerations:
  * Have there been changes to platform lifecycle events or background task behaviors (e.g., 0-duration pings, or ping submission issues)?
  * Has the OS changed the behaviour of system APIs?

### 7\. Time Differences: start/end\_time vs. submission\_timestamp

* Purpose: Assess the delay between telemetry collection and submission.
* Column Names: `ping_info.parsed_start_time`, `ping_info.parsed_end_time`, `submission_timestamp`
* Considerations:
  * Are the recorded timestamps reasonable, both in terms of the ping time window and the delay from collection to submission?

### 8\. Glean Errors

* Purpose: Identify [telemetry or network errors](../../metrics/error-reporting.md) related to data collection.
* Considerations:
  * Are there networking errors, ingestion issues, or other telemetry failures that could be related to the anomaly?

### 9\. Hardware Details (Manufacturer/Model) (Mobile platforms only)

* Purpose: Determine if the issue is hardware-specific.
* Column Names: `client_info.device_manufacturer`, `client_info.device_model`
* Considerations:
  * Does the anomaly occur primarily on older or newer hardware models?

### 10\. Build Details: Architecture

* Purpose: Determine if the issue is specific to a class of hardware or build configuration.
* Column Name: `client_info.architecture`
* Considerations:
  * Are affected clients only running builds built for a specific architecture?
  * Are all clients running builds built for a specific architecture affected?
  * Has the build configuration for this architecture changed recently?

### 11\. Ping reason

* Purpose: Determine the reason a ping was sent.
* Column Names: `ping_info.reason`
* Considerations:
  * Does the anomaly occur primarily for a specific reason?
  * The built-in pings have different ping reasons based on their schedule
    * [`baseline` ping schedule and reasons](../../pings/baseline.md#scheduling)
    * [`metrics` ping schedule and reasons](../../pings/metrics.md#scheduling)
    * [`events` ping schedule and reasons](../../pings/events.md#scheduling)

### 12\. No Data

* Purpose: Determine why a metric isn't included in a ping in which it is expected to be found.
* Considerations:
  * Is the metric defined to be included in the ping/dataset via `send_in_pings`, or should it be on the ping by default?
    * Check the metric definition in the `metrics.yaml` file using the links provided in the [Glean Dictionary](https://dictionary.telemetry.mozilla.org/).
    * Check to see that the specific metric API is being used in an active code path to ensure that the metric is being recorded (is `set`, `record`, `accumulateSamples`, etc. being called).
    * Check [Experimenter](https://experimenter.services.mozilla.com/) for any experiments or rollouts that might be using Server Knobs to sample the metric.
    * Validate the recording locally using the [Glean Debug tools](../../debugging/index.md), checking for the metric to be included in the expected pings in either the logs or the Glean Debug View.
    * Ensure you are filtering for only versions with the metric if this is a newly added instrumentation, this can help isolate other issues.
  * Is the metric recorded some of the time but missing or null at other times?
    * Ensure the metric definition has an appropriate metric lifetime. Should a metric be defined with a `ping` lifetime, but not be recorded before the next ping is submitted, it is expected to be missing. If you want Glean to cache the metric differently, refer to the lifetime information in the documentation for [metrics](../../metrics/adding-new-metrics.md).
    * Check for [Glean errors](../../metrics/error-reporting.md) related to the metric. It is possible that the instrumentation is attempting to record something that does not pass Glean validation which could cause intermittent missing values.
    * Is this uniform across the population (using the slices suggested above)? If not, check for a feature not being available to certain populations, or an experiment or rollout which could be affecting the instrumentation.
