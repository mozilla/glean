# Implementations

| Project Name      | Language Bindings   | Operating System  | App Lifecycle Type  | Environment Data source |
| ----------------- | ------------------- | ----------------  | ------------------- | ----------------------- |
| glean-core        | Rust                | all               | all                 | none                    |
| glean-ffi         | C                   | all               | all                 | none                    |
| glean             | Rust                | Windows/Mac/Linux | Desktop application | OS info build-time autodetected, app info passed in |
| Glean Android     | Kotlin, Java        | Android           | Mobile app          | Autodetected from the Android environment |
| Glean iOS         | Swift               | iOS               | Mobile app          | Autodetected from the iOS environment
| Glean.py          | Python              | Windows/Mac/Linux | all                 | Autodetected at runtime |
| FOG[^1]           | Rust/C++/JavaScript | as Firefox supports | Desktop application | OS info build-time autodetected, app info passed in |

## Features matrix

| Feature/Bindings        | Kotlin   | Swift  | Python | Rust |
| ----------------------- | -------- | ------ | ------ | ---- |
| Core metric types       | ✓        | ✓      | ✓      | ✓    |
| Metrics Testing API     | ✓        | ✓      | ✓      | ✓    |
| `baseline` ping         | ✓        | ✓      | X      | X    |
| `metrics`               | ✓        | ✓      | X      | X    |
| `events`                | ✓        | ✓      | ✓      | X    |
| `deletion-request` ping | ✓        | ✓      | ✓      | ✓    |
| Custom pings            | ✓        | ✓      | ✓      | ✓    |
| Custom pings testing API| ✓        | ✓      | ✓      | X    |
| Debug Ping View support | ✓        | ✓      | ✓      | ✓    |

---

[^1]: [Firefox on Glean (FOG)](https://firefox-source-docs.mozilla.org/toolkit/components/glean/index.html) is the name of the layer that integrates the Glean SDK into Firefox Desktop. It uses the Glean Rust bindings and exposes the same Rust API inside Firefox and extends it with a C++ and JavaScript API.
