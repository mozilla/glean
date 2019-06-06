// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::error::Result;
use crate::Glean;

/// Stores information about a ping.
///
/// This is required so that given metric data queued on disk we can send
/// pings with the correct settings, e.g. whether it has a client_id.
#[derive(Clone, Debug)]
pub struct PingType {
    /// The name of the ping.
    pub name: String,
    /// Whether the ping should include the client_id data
    pub include_client_id: bool,
}

impl PingType {
    pub fn new<A: Into<String>>(name: A, include_client_id: bool) -> Self {
        Self {
            name: name.into(),
            include_client_id,
        }
    }

    pub fn send(&self, glean: &Glean, log_ping: bool) -> Result<bool> {
        glean.send_ping(self, log_ping)
    }
}
