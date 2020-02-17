# The `deletion-request` ping

## Description

This ping is submitted when a user opts out of sending technical and interaction data.

This ping contains the client id.

This ping is intended to communicate to the Data Pipeline that the user wishes to have their reported Telemetry data deleted.
As such it attempts to send itself at the moment the user opts out of data collection, and continues to try and send itself.

## Scheduling

The `deletion-request` ping is automatically submitted when upload is disabled in Glean.
If upload fails, it is retried after Glean is initialized.

## Contents

The `deletion-request` does not contain additional metrics.

## Example `deletion-request` ping

```json
{
  "ping_info": {
    "seq": 0,
    "start_time": "2019-12-06T09:50-04:00",
    "end_time": "2019-12-06T09:53-04:00"
  },
  "client_info": {
    "telemetry_sdk_build": "22.0.0",
    "first_run_date": "2019-03-29-04:00",
    "os": "Android",
    "android_sdk_version": "28",
    "os_version": "9",
    "device_manufacturer": "Google",
    "device_model": "Android SDK built for x86",
    "architecture": "x86",
    "app_build": "1",
    "app_display_version": "1.0",
    "client_id": "35dab852-74db-43f4-8aa0-88884211e545"
  }
}
```
