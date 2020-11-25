// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use inherent::inherent;

/// A Glean ping.
#[derive(Clone, Debug)]
pub struct PingType {
    pub(crate) name: String,
    pub(crate) ping_type: glean_core::metrics::PingType,
}

impl PingType {
    /// Creates a new ping type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the ping.
    /// * `include_client_id` - Whether to include the client ID in the assembled ping when.
    /// * `send_if_empty` - Whether the ping should be sent empty or not.
    /// * `reason_codes` - The valid reason codes for this ping.
    pub fn new<A: Into<String>>(
        name: A,
        include_client_id: bool,
        send_if_empty: bool,
        reason_codes: Vec<String>,
    ) -> Self {
        let name = name.into();
        let ping_type = glean_core::metrics::PingType::new(
            name.clone(),
            include_client_id,
            send_if_empty,
            reason_codes,
        );

        let me = Self { name, ping_type };
        crate::register_ping_type(&me);
        me
    }
}

#[inherent(pub)]
impl glean_core::traits::Ping for PingType {
    /// Collect and submit the ping for eventual upload.
    ///
    /// This will collect all stored data to be included in the ping.
    /// Data with lifetime `ping` will then be reset.
    ///
    /// If the ping is configured with `send_if_empty = false`
    /// and the ping currently contains no content,
    /// it will not be queued for upload.
    /// If the ping is configured with `send_if_empty = true`
    /// it will be queued for upload even if otherwise empty.
    ///
    /// Pings always contain the `ping_info` and `client_info` sections.
    /// See [ping sections](https://mozilla.github.io/glean/book/user/pings/index.html#ping-sections)
    /// for details.
    ///
    /// # Arguments
    ///
    /// * `reason` - The reason the ping is being submitted.
    ///              Must be one of the configured `reason_codes`.
    fn submit(&self, reason: Option<&str>) {
        crate::submit_ping(self, reason)
    }
}
