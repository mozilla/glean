# Implementations

| Project Name      | Language Bindings   | Operating System  | App Lifecycle Type  | Environment Data source |
| ----------------- | ------------------- | ----------------  | ------------------- | ----------------------- |
| glean-core        | Rust                | all               | all                 | none                    |
| glean             | Rust                | Windows/Mac/Linux | Desktop application | OS info build-time autodetected, app info passed in |
| Glean Android     | Kotlin, Java        | Android           | Mobile app          | Autodetected from the Android environment |
| Glean iOS         | Swift               | iOS               | Mobile app          | Autodetected from the iOS environment
| Glean.py          | Python              | Windows/Mac/Linux | all                 | Autodetected at runtime |
| FOG[^1]           | Rust/C++/JavaScript | as Firefox supports | Desktop application | OS info build-time autodetected, app info passed in |

## Features matrix

| Feature/Bindings        | Kotlin | Swift | Python | Rust |
| ----------------------- | ------ | ----- | ------ | ---- |
| Core metric types       | ✅     | ✅    | ✅      | ✅     |
| Metrics Testing API     | ✅     | ✅    | ✅      | ✅     |
| `baseline` ping         | ✅     | ✅    | ✅[^2]  | ✅[^3] |
| `metrics`               | ✅     | ✅    | ❌      | ✅[^4] |
| `events`                | ✅     | ✅    | ✅      | ✅[^5] |
| `deletion-request` ping | ✅     | ✅    | ✅      | ✅     |
| Custom pings            | ✅     | ✅    | ✅      | ✅     |
| Custom pings testing API| ✅     | ✅    | ✅      | ✅     |
| Debug Ping View support | ✅     | ✅    | ✅      | ✅     |

---

[^1]: [Firefox on Glean (FOG)](https://firefox-source-docs.mozilla.org/toolkit/components/glean/index.html) is the name of the layer that integrates the Glean SDK into Firefox Desktop. It uses the Glean Rust bindings and exposes the same Rust API inside Firefox and extends it with a C++ and JavaScript API.

[^2]: Not sent automatically. Use the [`handle_client_active`][py_client_active] and [`handle_client_inactive`][py_client_inactive] API.

[^3]: Sent automatically on startup if necessary. For active/inactive pings use the [`handle_client_active`][rs_client_active] and [`handle_client_inactive`][rs_client_inactive] API.

[^4]: Needs to be enabled using `use_core_mps` in the [`Configuration`][rs_configuration].

[^5]: Sent on startup when pending events are stored and when reaching the limit. Additionally sent when [`handle_client_inactive`][rs_client_inactive] is called.


[py_client_active]: ../../../python/glean/#glean.Glean.handle_client_active
[py_client_inactive]: ../../../python/glean/#glean.Glean.handle_client_inactive
[rs_client_active]: ../../../docs/glean/fn.handle_client_active.html
[rs_client_inactive]: ../../../docs/glean/fn.handle_client_inactive.html
[rs_configuration]: ../../../docs/glean/struct.Configuration.html
