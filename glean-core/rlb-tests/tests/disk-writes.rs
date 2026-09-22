// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::sync::mpsc::{Receiver, channel};

use notify::{Event, RecursiveMode, Watcher};
use xshell::{Shell, cmd};

#[derive(Debug, Default)]
struct EventStats {
    total: usize,
    modify: usize,
    create: usize,
    access: usize,
    remove: usize,
    renames: usize,
    metadata: usize,
    writes: usize,
}

fn summarize_stats(rx: Receiver<notify::Result<Event>>, data_dir: &Path) -> BTreeMap<String, EventStats> {
    let mut stats: BTreeMap<String, EventStats> = BTreeMap::new();

    for res in rx {
        let event = match res {
            Ok(event) => event,
            Err(e) => {
                println!("watch error: {:?}", e);
                continue;
            },
        };

        let mut paths = event.paths;
        assert!(1 <= paths.len(), "{:?}", paths);
        let path = paths.remove(0);

        let path = path.strip_prefix(&data_dir).unwrap();
        if path.display().to_string() == "db" {
            continue;
        }

        let stat = stats.entry(path.display().to_string()).or_default();
        stat.total += 1;

        match event.kind {
            notify::EventKind::Any => todo!(),
            notify::EventKind::Access(_) => stat.access += 1,
            notify::EventKind::Create(_) => stat.create += 1,
            notify::EventKind::Modify(modify_kind) => {
                match modify_kind {
                    notify::event::ModifyKind::Any => todo!(),
                    notify::event::ModifyKind::Data(_) => {
                        stat.writes += 1;
                    },
                    notify::event::ModifyKind::Metadata(_) => stat.metadata += 1,
                    notify::event::ModifyKind::Name(_) => stat.renames += 1,
                    notify::event::ModifyKind::Other => todo!(),
                }
                stat.modify += 1
            },
            notify::EventKind::Remove(_) => stat.remove += 1,
            notify::EventKind::Other => todo!(),
        }
    }

    stats
}

fn run_rapid_metrics<F: Into<Option<&'static str>>, A: Into<Option<&'static str>>>(sh: &Shell, feature: F, extra_arg: A, maxn: usize, seed: usize) -> usize {
    let feature = feature.into().map(|a| format!("--features={a}"));
    let extra_arg = extra_arg.into();


    let tempdir = tempfile::tempdir().unwrap();
    let db_dir = tempdir.path().join("db");
    fs::create_dir_all(&db_dir).unwrap();

    let (tx, rx) = channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx).unwrap();
    watcher.watch(&db_dir, RecursiveMode::Recursive).unwrap();

    let maxn = maxn.to_string();
    let seed = seed.to_string();
    let data_dir = tempdir.path().canonicalize().unwrap();

    cmd!(sh, "cargo run -q --release -p rapid-metrics {feature...} -- {extra_arg...} --maxn {maxn} --seed {seed} {data_dir}").run().unwrap();

    drop(watcher);

    println!("Path\tTotal");
    let mut total = 0;
    let stats = summarize_stats(rx, &data_dir);
    for (path, stats) in &stats {
        println!("{}\t{}", path, stats.total);
        total += stats.total;
    }

    total
}

/// Compare disk writes in Rkv and SQLite mode.
///
/// We count all file events to any files in the `db/` directory.
/// The assumption is that SQLite is strictly better than the equivalent mode with Rkv.
///
/// Two environment variables control the test:
///
/// * `MAXN` -- how many iterations `rapid-metrics` does. Defaults to 100.
/// * `SEED` -- the seed to base the rng on. Defaults to 100.
#[ignore]
#[test]
fn compare_rkv_sqlite_writes() {
    let sh = Shell::new().unwrap();

    let maxn = std::env::var("MAXN").unwrap_or_else(|_| String::new()).parse().unwrap_or(100);
    let seed = std::env::var("SEED").unwrap_or_else(|_| String::new()).parse().unwrap_or(100);

    let rkv_full = run_rapid_metrics(&sh, None, None, maxn, seed);
    let rkv_delay = run_rapid_metrics(&sh, None, "--delay", maxn, seed);
    let sqlite_full = run_rapid_metrics(&sh, "sqlite", None, maxn, seed);
    let sqlite_delay = run_rapid_metrics(&sh, "sqlite", "--delay", maxn, seed);

    // Full always takes strictly more writes than delay
    assert!(rkv_full > rkv_delay, "rkv_full={rkv_full} > rkv_delay={rkv_delay}");

    #[cfg(target_os = "linux")]
    assert!(sqlite_full > sqlite_delay, "sqlite_full={sqlite_full} > sqlite_delay={sqlite_delay}");

    // The fs notification on macOS are not as precise,
    // impacting what we measure (next to nothing it seems).
    #[cfg(target_os = "macos")]
    assert!(sqlite_full >= sqlite_delay, "sqlite_full={sqlite_full} > sqlite_delay={sqlite_delay}");

    // Rkv in full mode writes strictly more than SQLite
    assert!(rkv_full > sqlite_full, "rkv_full={rkv_full} > sqlite_delay={sqlite_delay}");

    // Rkv in delay mode writes more than SQLite in delay mode
    assert!(rkv_delay > sqlite_delay, "rkv_delay={rkv_delay} > sqlite_delay={sqlite_delay}");
}
