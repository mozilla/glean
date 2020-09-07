# Implementations

| Project Name      | Language Bindings   | Operating System  | App Lifecycle Type  | Environment Data source |
| ----------------- | ------------------- | ----------------  | ------------------- | ----------------------- |
| glean-core        | Rust                | all               | all                 | none                    |
| glean-ffi         | C                   | all               | all                 | none                    |
| glean-preview[^1] | Rust                | Windows/Mac/Linux | Desktop application | OS info build-time autodetected, app info passed in |
| Glean Android     | Kotlin, Java        | Android           | Mobile app          | Autodetected from the Android environment |
| Glean iOS         | Swift               | iOS               | Mobile app          | Autodetected from the iOS environment
| Glean.py          | Python              | Windows/Mac/Linux | all                 | Autodetected at runtime |
| FOG[^2]           | Rust/C++/JavaScript | as Firefox supports | Desktop application | OS info build-time autodetected, app info passed in |

## Features matrix

| Feature/Bindings        | Kotlin   | Swift  | Python  | C# | Rust |
| ----------------------- | -------- | ------ | ------- | -- | ---- |
| Core metric types       | ✓        | ✓      | ✓      | ✓  | X    |
| Metrics Testing API     | ✓        | ✓      | ✓      | ✓  | X    |
| `baseline` ping         | ✓        | ✓      | X      | X  | X    |
| `metrics`               | ✓        | ✓      | X      | X  | X    |
| `events`                | ✓        | ✓      | ✓      | ✓  | X    |
| `deletion-request` ping | ✓        | ✓      | ✓      | ✓  | ✓   |
| Custom pings            | ✓        | ✓      | ✓      | ✓  | X    |
| Custom pings testing API| ✓        | ✓      | ✓      | X  | X    |
| Debug Ping View support | ✓        | ✓      | ✓      | ✓  | ✓   |

---

[^1]: [glean-preview](https://crates.io/crates/glean-preview) is an experimental crate for prototyping integration into Firefox. It it not recommended for general use. See Project FOG.

[^2]: [Firefox on Glean (FOG)](https://firefox-source-docs.mozilla.org/toolkit/components/glean/index.html) is the name of the layer that integrates the Glean SDK into Firefox Desktop. It is currently being designed and implemented. It is being used as a test bed for how best to write a generic Rust language binding layer and so temporarily ties directly to glean-core instead of an API crate. Prospective non-mozilla-central Rust consumers of the Glean SDK should not follow its example and should instead follow [bug 1631768](https://bugzilla.mozilla.org/show_bug.cgi?id=1631768) for updates on when the proper language binding crate will be available.
