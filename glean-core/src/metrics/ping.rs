// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::error::Result;
use crate::Glean;

#[derive(Debug, Clone)]
pub struct PingType {
    pub name: String,
    pub include_client_id: bool,
}

impl PingType {
    pub fn new<A: Into<String> + Copy>(name: A, include_client_id: bool) -> Self {
        Self {
            name: name.into(),
            include_client_id,
        }
    }

    pub fn send(&self, glean: &Glean, log_ping: bool) -> Result<bool> {
        glean.send_ping(self, log_ping)
    }
}
