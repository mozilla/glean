# Implementations

| Project Name      | Language Bindings   | Operating System  | App lifecycle type  | Environment Data source |
| ----------------- | ------------------- | ----------------  | ------------------- | ----------------------- |
| glean-core        | Rust                | all               | all                 | none                    |
| glean-ffi         | C                   | all               | all                 | none                    |
| glean-preview[^1] | Rust                | Windows/Mac/Linux | Desktop application | OS info build-time autodetected, app info passed in |
| Glean Android     | Kotlin, Java        | Android           | Mobile app          | Autodetected from the Android environment |
| Glean iOS         | Swift               | iOS               | Mobile app          | Autodetected from the iOS environment
| Glean.py          | Python              | Windows/Mac/Linux | all                 | Autodetected at runtime |
| FOG[^2]           | Rust/C++/JavaScript | as Firefox supports | Desktop application | OS info build-time autodetected, app info passed in |

---

[^1]: [glean-preview](https://crates.io/crates/glean-preview) is an experimental crate for prototyping integration into Firefox. It it not recommended for general use. See Project FOG.

[^2]: [Firefox on Glean (FOG)](https://firefox-source-docs.mozilla.org/toolkit/components/glean/index.html) is the name of the layer that integrates the Glean SDK into Firefox Desktop. It is currently being designed and implemented.
