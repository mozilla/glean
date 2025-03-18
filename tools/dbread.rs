#!/usr/bin/env -S cargo +nightly -q -Zscript
---cargo
[dependencies]
bincode = "1.0"
serde = {version = "1.0.144", features = ["derive"]}
bitflags = {version = "2.4.1", features = ["serde"]}
tabwriter = "1.1.0"
---

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! A tiny tool to read Rkv-based (safe-mode) databases as produced by Glean.
//!
//! Displays the metrics in every store, more complex metric types (e.g. distributions)
//! are not displayed in full.

use std::env;
use std::io::{self, Write};
use std::collections::{HashMap, BTreeMap};

use bitflags::bitflags;
use serde::Deserialize;
use tabwriter::TabWriter;

bitflags! {
    #[derive(Default, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
    pub struct DatabaseFlagsImpl: u32 {
        const NIL = 0b0000_0000;
    }
}

type Key = Box<[u8]>;
type Value = Box<[u8]>;

#[derive(Debug, Clone, Deserialize)]
pub struct Snapshot {
    _flags: DatabaseFlagsImpl,
    map: BTreeMap<Key, Value>,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    snapshot: Snapshot,
}

/// Reduced duplicate of `enum Metric` in `glean-core/src/metrics/mod.rs`
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum Metric {
    Boolean(bool),
    Counter(i32),
    CustomDistributionExponential,
    CustomDistributionLinear,
    Datetime,
    Experiment,
    Quantity(i64),
    String(String),
    StringList(Vec<String>),
    Uuid(String),
    Timespan,
    TimingDistribution,
    MemoryDistribution,
    Jwe(String),
    Rate(i32, i32),
    Url(String),
    Text(String),
    Object(String),
}

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().expect("/path/to/db");
    let content = std::fs::read(path).unwrap();

    let data: HashMap<Option<String>, Database> = bincode::deserialize(&content).unwrap();
    let mut data: Vec<_> = data.into_iter().collect();
    data.sort_by(|a, b| a.0.cmp(&b.0));
    let stdout = io::stdout();

    for (name, db) in data {
        let handle = stdout.lock();
        let mut tw = TabWriter::new(handle);
        let name = name.unwrap_or(String::from(""));
        writeln!(&mut tw, "DB: {name}").unwrap();
        writeln!(&mut tw, "Key\tValue").unwrap();
        writeln!(&mut tw, "===\t=====").unwrap();

        for (k, v) in db.snapshot.map {
            let k = str::from_utf8(&k).unwrap();
            if v[0] == 9 {
                let d: Vec<u8> = bincode::deserialize(&v[1..]).unwrap();
                let m: Metric = bincode::deserialize(&d).unwrap();
                writeln!(&mut tw, "{k}\t{m:?}").ok();
            } else {
                writeln!(&mut tw, "{k}\t{:?}..", &v[..4]).ok();
            }
        }
        writeln!(&mut tw, "").ok();
        tw.flush().unwrap();
    }
}
