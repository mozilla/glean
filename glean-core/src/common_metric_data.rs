// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::convert::TryFrom;

use crate::error::{Error, ErrorKind};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Lifetime {
    /// The metric is reset with each sent ping
    Ping,
    /// The metric is reset on application restart
    Application,
    /// The metric is reset with each user profile
    User,
}

impl Default for Lifetime {
    fn default() -> Self {
        Lifetime::Ping
    }
}

impl Lifetime {
    pub fn as_str(self) -> &'static str {
        match self {
            Lifetime::Ping => "ping",
            Lifetime::Application => "app",
            Lifetime::User => "user",
        }
    }
}

impl TryFrom<i32> for Lifetime {
    type Error = Error;

    fn try_from(value: i32) -> Result<Lifetime, Self::Error> {
        match value {
            0 => Ok(Lifetime::Ping),
            1 => Ok(Lifetime::Application),
            2 => Ok(Lifetime::User),
            e => Err(ErrorKind::Lifetime(e))?,
        }
    }
}

#[derive(Default, Debug)]
pub struct CommonMetricData {
    pub name: String,
    pub category: String,
    pub send_in_pings: Vec<String>,
    pub lifetime: Lifetime,
    pub disabled: bool,
}

impl CommonMetricData {
    /// Create a new metadata object
    pub fn new<A: Into<String>, B: Into<String>, C: Into<String>>(
        category: A,
        name: B,
        ping_name: C,
    ) -> CommonMetricData {
        CommonMetricData {
            name: name.into(),
            category: category.into(),
            send_in_pings: vec![ping_name.into()],
            ..Default::default()
        }
    }

    pub fn identifier(&self) -> String {
        if self.category.is_empty() {
            self.name.clone()
        } else {
            format!("{}.{}", self.category, self.name)
        }
    }

    pub fn should_record(&self) -> bool {
        if self.disabled {
            return false;
        }
        true
    }

    pub fn storage_names(&self) -> &[String] {
        &self.send_in_pings
    }
}
