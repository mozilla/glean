use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::{Command, cargo_bin, cargo_bin_cmd};

fn collect_files(path: &Path) -> Vec<PathBuf> {
    fs::read_dir(path)
        .unwrap()
        .map(|res| res.unwrap().path())
        .collect::<Vec<_>>()
}

#[test]
fn shutdown_blocking() {
    let tempdir = tempfile::tempdir().unwrap();

    cargo_bin_cmd!("long-running")
        .arg(tempdir.path())
        .assert()
        .success();

    let entries = collect_files(&tempdir.path().join("pending_pings"));
    assert_eq!(1, entries.len());
}

#[test]
fn thread_crashing() {
    let tempdir = tempfile::tempdir().unwrap();

    cargo_bin_cmd!("crashing-threads")
        .arg(tempdir.path())
        .assert()
        .success();

    let entries = collect_files(&tempdir.path().join("pending_pings"));

    // 1x "health", 1x "prototype"
    assert_eq!(2, entries.len());
}

#[test]
fn delayed_ping_data() {
    let tempdir = tempfile::tempdir().unwrap();

    // First run "crashes" -> no increment stored
    cargo_bin_cmd!("delayed-ping-data")
        .arg(tempdir.path())
        .arg("accumulate_one_and_pretend_crash")
        .assert()
        .success();

    let sent_pings_dir = tempdir.path().join("sent_pings");

    let entries = collect_files(&sent_pings_dir);
    assert_eq!(0, entries.len());

    // Second run increments and orderly shuts down -> increment flushed to disk.
    // No ping is sent.
    cargo_bin_cmd!("delayed-ping-data")
        .arg(tempdir.path())
        .arg("accumulate_ten_and_orderly_shutdown")
        .assert()
        .success();

    let entries = collect_files(&sent_pings_dir);
    assert_eq!(0, entries.len());

    // Third run sends the ping.
    cargo_bin_cmd!("delayed-ping-data")
        .arg(tempdir.path())
        .arg("submit_ping")
        .assert()
        .success();

    let entries = collect_files(&sent_pings_dir);
    assert_eq!(1, entries.len());
    let payload = fs::read_to_string(&entries[0]).unwrap();
    let mut lines = payload.lines();
    assert!(lines.any(|line| line.contains("\"url\":") && line.contains("/prototype/")));
    assert!(payload.contains("\"test.metrics.sample_counter\":10"));
}

#[cfg(unix)]
#[test]
fn mps_delay() {
    // On dev machines check for `faketime` and skip the test if it is missing.
    // On CI hard-fail if `faketime is missing.
    if std::env::var("CI").is_err() {
        let mut cmd = Command::new("command");
        cmd.args(["-v", "faketime"]);
        let result = cmd.assert().try_success();
        if result.is_err() {
            // Unfortunately Rust's test harness doesn't have a way to mark a test as skipped.
            eprintln!("SKIPPING. faketime is missing.");
            return;
        }
    }

    let tempdir = tempfile::tempdir().unwrap();
    let envs = [
        ("TZ", "America/Los_Angeles"), // -07:00
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    let exe = cargo_bin!("mps-delay");
    let sent_pings_dir = tempdir.path().join("sent_pings");

    let mut cmd = Command::new("timeout");
    cmd.envs(&envs)
        .arg("5s")
        .arg("faketime")
        .arg("--exclude-monotonic")
        .args(["-f", "2025-07-27 04:05:00"])
        .arg(exe)
        .arg(tempdir.path())
        .arg("init");
    cmd.assert().success();

    let pings = collect_files(&sent_pings_dir);
    assert_eq!(0, pings.len());

    let mut cmd = Command::new("timeout");
    cmd.envs(&envs)
        .arg("5s")
        .arg("faketime")
        .arg("--exclude-monotonic")
        .args(["-f", "2025-07-28 22:27:00"])
        .arg(exe)
        .arg(tempdir.path())
        .arg("second");
    cmd.assert().success();

    let pings = collect_files(&sent_pings_dir);
    assert_eq!(1, pings.len());

    let mut cmd = Command::new("timeout");
    cmd.envs(&envs)
        .arg("5s")
        .arg("faketime")
        .arg("--exclude-monotonic")
        .args(["-f", "2025-07-28 22:30:00"])
        .arg(exe)
        .arg(tempdir.path())
        .arg("third");
    cmd.assert().success();

    let pings = collect_files(&sent_pings_dir);
    assert_eq!(1, pings.len());
}

#[test]
fn ping_lifetime_flush() {
    let tempdir = tempfile::tempdir().unwrap();
    let sent_pings_dir = tempdir.path().join("sent_pings");

    // First run "crashes" -> no increment stored
    cargo_bin_cmd!("ping-lifetime-flush")
        .arg(tempdir.path())
        .arg("accumulate_one_and_pretend_crash")
        .assert()
        .success();

    let pings = collect_files(&sent_pings_dir);
    assert_eq!(0, pings.len());

    // Second run increments, waits, increments -> increment flushed to disk.
    // No ping is sent.
    cargo_bin_cmd!("ping-lifetime-flush")
        .arg(tempdir.path())
        .arg("accumulate_ten_and_wait")
        .assert()
        .success();

    let pings = collect_files(&sent_pings_dir);
    assert_eq!(0, pings.len());

    // Third run sends the ping.
    cargo_bin_cmd!("ping-lifetime-flush")
        .arg(tempdir.path())
        .arg("submit_ping")
        .assert()
        .success();

    let entries = collect_files(&sent_pings_dir);
    assert_eq!(1, entries.len());
    let payload = fs::read_to_string(&entries[0]).unwrap();
    let mut lines = payload.lines();
    assert!(lines.any(|line| line.contains("\"url\":") && line.contains("/prototype/")));
    assert!(payload.contains("\"test.metrics.sample_counter\":20"));
}

#[test]
fn pending_gets_removed() {
    let tempdir = tempfile::tempdir().unwrap();
    let pending_pings_dir = tempdir.path().join("pending_pings");

    cargo_bin_cmd!("pending-gets-removed")
        .arg(tempdir.path())
        .arg("1")
        .assert()
        .success();

    let expected = ["validation", "nofollows", "health"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();

    let entries = collect_files(&pending_pings_dir);
    let mut found = HashSet::new();

    for ping in entries.into_iter() {
        let payload = fs::read_to_string(&ping).unwrap();
        let mut lines = payload.lines();
        let mut parts = lines.next().unwrap().split("/");
        assert_eq!("", parts.next().unwrap());
        assert_eq!("submit", parts.next().unwrap());
        assert_eq!("glean-pending-removed", parts.next().unwrap());
        assert!(found.insert(parts.next().unwrap().to_string()));
    }
    assert_eq!(expected, found);

    cargo_bin_cmd!("pending-gets-removed")
        .arg(tempdir.path())
        .arg("2")
        .assert()
        .success();
    let entries = collect_files(&pending_pings_dir);
    assert_eq!(1, entries.len());
    let payload = fs::read_to_string(&entries[0]).unwrap();
    assert!(payload.contains("/nofollows/"));

    cargo_bin_cmd!("pending-gets-removed")
        .arg(tempdir.path())
        .arg("3")
        .assert()
        .success();
    let entries = collect_files(&pending_pings_dir);
    assert_eq!(0, entries.len());
}

#[test]
fn enabled_pings() {
    let tempdir = tempfile::tempdir().unwrap();
    let sent_pings_dir = tempdir.path().join("sent_pings");

    cargo_bin_cmd!("enabled-pings")
        .arg(tempdir.path())
        .arg("default")
        .assert()
        .success();
    let entries = collect_files(&sent_pings_dir);
    assert_eq!(1, entries.len());
    let payload = fs::read_to_string(&entries[0]).unwrap();
    assert!(payload.contains("/one/"), "Payload: {payload}");

    let tempdir = tempfile::tempdir().unwrap();
    let sent_pings_dir = tempdir.path().join("sent_pings");
    cargo_bin_cmd!("enabled-pings")
        .arg(tempdir.path())
        .arg("enable-both")
        .assert()
        .success();
    let expected = ["one", "two"]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();

    let entries = collect_files(&sent_pings_dir);
    let mut found = HashSet::new();

    for ping in entries.into_iter() {
        let payload = fs::read_to_string(&ping).unwrap();
        let mut lines = payload.lines();
        _ = lines.next().unwrap();
        let mut parts = lines.next().unwrap().split("/");
        assert_ne!("", parts.next().unwrap());
        assert_eq!("submit", parts.next().unwrap());
        assert_eq!("glean-enabled-pings", parts.next().unwrap());
        assert!(found.insert(parts.next().unwrap().to_string()));
    }
    assert_eq!(expected, found);

    let tempdir = tempfile::tempdir().unwrap();
    let sent_pings_dir = tempdir.path().join("sent_pings");
    cargo_bin_cmd!("enabled-pings")
        .arg(tempdir.path())
        .arg("enable-only-two")
        .assert()
        .success();

    let entries = collect_files(&sent_pings_dir);
    assert_eq!(1, entries.len());
    let payload = fs::read_to_string(&entries[0]).unwrap();
    assert!(payload.contains("/two/"), "Payload: {payload}");
}

#[test]
fn rkv_sqlite_migration() {
    let tempdir = tempfile::tempdir().unwrap();

    let db_dir = tempdir.path().join("db");
    fs::create_dir_all(&db_dir).unwrap();
    let rkv_db = db_dir.join("data.safe.bin");
    fs::write(&rkv_db, include_bytes!("rkv-database.safe.bin")).unwrap();

    cargo_bin_cmd!("verify-data")
        .arg(tempdir.path())
        .arg("verify")
        .assert()
        .success();
}
