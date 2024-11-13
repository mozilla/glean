#![cfg_attr(rustfmt, rustfmt_skip)]

use std::io::Write;
use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};

use glean_build::Builder;

struct Metric {
    name: String,
    typ: String,
}

struct Ping {
    name: String,
}

fn main() {
    Builder::default()
        .file("metrics.yaml")
        .file("pings.yaml")
        .generate()
        .expect("Error generating Glean Rust bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let glean_metrics = out_dir.join("glean_metrics.rs");
    let content = fs::read_to_string(&glean_metrics).unwrap();

    let info_file_path = out_dir.join("metric_info.rs");
    let info_file = File::create(info_file_path).unwrap();

    let mut metrics = vec![];
    let mut pings = vec![];
    for line in content.lines() {
        if !line.contains("__export::Lazy") {
            continue;
        }
        if !line.contains("pub static") {
            continue;
        }

        if line.contains("Metric") {
            let mut components = line.trim().split(" ");

            components.next().unwrap();
            components.next().unwrap();
            let name = components.next().unwrap().trim_matches(':').to_string();
            let typ = components
                .next()
                .unwrap()
                .strip_prefix("::glean::private::__export::Lazy<")
                .unwrap()
                .strip_suffix(">")
                .unwrap()
                .to_string();
            metrics.push(Metric { name, typ });
        }

        if line.contains("PingType") {
            let mut components = line.trim().split(" ");

            components.next().unwrap();
            components.next().unwrap();
            let name = components.next().unwrap().trim_matches(':').to_string();

            pings.push(Ping { name });
        }
    }

    writeln!(&info_file, "pub static METRICS: &[&str] = &[").unwrap();
    for metric in &metrics {
        writeln!(&info_file, "{:?}, // {:?}", metric.name, metric.typ).unwrap();
    }
    writeln!(&info_file, "];").unwrap();

    writeln!(&info_file, "pub fn metric_get(name: &str, ping: Option<String>) {{").unwrap();
    writeln!(&info_file, "match name {{").unwrap();
    for metric in &metrics {
        writeln!(&info_file, "{:?} => {{", metric.name).unwrap();
        if metric.typ.contains("Event") {
        } else {
            writeln!(&info_file, "println!(\"Value: {{:?}}\", super::glean_metrics::metrics::{}.test_get_value(ping.into()));", metric.name).unwrap();
        }
        writeln!(&info_file, "}}").unwrap();
    }
    writeln!(&info_file, "other => {{").unwrap();
    writeln!(&info_file, "eprintln!(\"unknown metric: {{}}\", other);").unwrap();
    writeln!(&info_file, "}}").unwrap();
    writeln!(&info_file, "}}").unwrap();
    writeln!(&info_file, "}}").unwrap();

    // COUNTER
    writeln!(&info_file, "pub fn counter_add(name: &str, amount: i32) {{").unwrap();
    writeln!(&info_file, "match name {{").unwrap();
    for metric in &metrics {
        if !metric.typ.contains("Counter") || metric.typ.contains("LabeledMetric") {
            continue;
        }

        writeln!(&info_file, "{:?} => {{", metric.name).unwrap();
        writeln!(&info_file, "super::glean_metrics::metrics::{}.add(amount);", metric.name).unwrap();
        writeln!(&info_file, "println!(\"Value: {{:?}}\", super::glean_metrics::metrics::{}.test_get_value(None));", metric.name).unwrap();
        writeln!(&info_file, "}}").unwrap();
    }
    writeln!(&info_file, "other => {{").unwrap();
    writeln!(&info_file, "eprintln!(\"unknown metric: {{}}\", other);").unwrap();
    writeln!(&info_file, "}}").unwrap();
    writeln!(&info_file, "}}").unwrap();
    writeln!(&info_file, "}}").unwrap();

    // BOOLEAN
    writeln!(&info_file, "pub fn boolean_set(name: &str, val: bool) {{").unwrap();
    writeln!(&info_file, "match name {{").unwrap();
    for metric in &metrics {
        if !metric.typ.contains("Boolean") || metric.typ.contains("LabeledMetric") {
            continue;
        }

        writeln!(&info_file, "{:?} => {{", metric.name).unwrap();
        writeln!(&info_file, "super::glean_metrics::metrics::{}.set(val);", metric.name).unwrap();
        writeln!(&info_file, "println!(\"Value: {{:?}}\", super::glean_metrics::metrics::{}.test_get_value(None));", metric.name).unwrap();
        writeln!(&info_file, "}}").unwrap();
    }
    writeln!(&info_file, "other => {{").unwrap();
    writeln!(&info_file, "eprintln!(\"unknown metric: {{}}\", other);").unwrap();
    writeln!(&info_file, "}}").unwrap();
    writeln!(&info_file, "}}").unwrap();
    writeln!(&info_file, "}}").unwrap();

    // EVENTS
    writeln!(&info_file, "pub fn event_record(name: &str) {{").unwrap();
    writeln!(&info_file, "match name {{").unwrap();
    for metric in &metrics {
        if !metric.typ.contains("EventMetric") {
            continue;
        }

        writeln!(&info_file, "{:?} => {{", metric.name).unwrap();
        writeln!(&info_file, "super::glean_metrics::metrics::{}.record(None);", metric.name).unwrap();
        writeln!(&info_file, "println!(\"Value: {{:?}}\", super::glean_metrics::metrics::{}.test_get_value(None));", metric.name).unwrap();
        writeln!(&info_file, "}}").unwrap();
    }
    writeln!(&info_file, "other => {{").unwrap();
    writeln!(&info_file, "eprintln!(\"unknown metric: {{}}\", other);").unwrap();
    writeln!(&info_file, "}}").unwrap();
    writeln!(&info_file, "}}").unwrap();
    writeln!(&info_file, "}}").unwrap();


    // PINGS

    writeln!(&info_file, "pub static PINGS: &[&str] = &[").unwrap();
    for ping in &pings {
        writeln!(&info_file, "{:?},", ping.name).unwrap();
    }
    writeln!(&info_file, "];").unwrap();

    writeln!(&info_file, "pub fn register_pings() {{").unwrap();
    for ping in &pings {
        writeln!(&info_file, "_ = &*super::glean_metrics::{};", ping.name).unwrap();
    }
    writeln!(&info_file, "}}").unwrap();

    writeln!(&info_file, "pub fn ping_submit(name: &str) {{").unwrap();
    writeln!(&info_file, "match name {{").unwrap();
    for ping in &pings {
        writeln!(&info_file, "{:?} => {{", ping.name).unwrap();
        writeln!(&info_file, "super::glean_metrics::{}.submit(None);", ping.name).unwrap();
        writeln!(&info_file, "println!(\"{} submitted.\");", ping.name).unwrap();
        writeln!(&info_file, "}}").unwrap();
    }
    writeln!(&info_file, "other => {{").unwrap();
    writeln!(&info_file, "eprintln!(\"unknown ping: {{}}\", other);").unwrap();
    writeln!(&info_file, "}}").unwrap();
    writeln!(&info_file, "}}").unwrap();
    writeln!(&info_file, "}}").unwrap();
}
