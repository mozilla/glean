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

### 9\. Hardware Details (Manufacturer/Version) (Mobile platforms only)

* Purpose: Determine if the issue is hardware-specific.
* Column Names: `client_info.device_manufacturer`, `client_info.device_model`
* Considerations:
  * Does the anomaly occur primarily on older or newer hardware models?

### 10\. Ping reason

* Purpose: Determine the reason a ping was sent.
* Column Names: `ping_info.reason`
* Considerations:
  * Does the anomaly occur primarily for a specific reason?
  * The built-in pings have different ping reasons based on their schedule
    * [`baseline` ping schedule and reasons](../../pings/baseline.md#scheduling)
    * [`metrics` ping schedule and reasons](../../pings/metrics.md#scheduling)
    * [`events` ping schedule and reasons](../../pings/events.md#scheduling)
