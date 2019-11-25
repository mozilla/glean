// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::metrics::PingType;

/// Glean-provided pings, all enabled by default.
///
/// These pings are defined in `glean-core/pings.yaml` and for now manually translated into Rust
/// code.
///
/// They are parsed and registered by the platform-specific wrappers, but might be used
/// Glean-internal directly.
#[derive(Debug)]
pub struct InternalPings {
    pub baseline: PingType,
    pub metrics: PingType,
    pub events: PingType,
}

impl InternalPings {
    pub fn new() -> InternalPings {
        InternalPings {
            baseline: PingType::new("baseline", true),
            metrics: PingType::new("metrics", true),
            events: PingType::new("events", true),
        }
    }
}
