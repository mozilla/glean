use std::collections::{BTreeSet, HashMap};
use std::fs;

use glean_core::metrics::RemoteSettingsConfig;
use yaml_rust2::YamlLoader;

/// Metrics that are defined in `metrics.yaml`,
/// but are not used as "normal" metrics in code for various reasons.
///
/// These will NOT be marked as errors.
static DEFINITION_ONLY: &[&str] = &[
    /* Adhoc used in code */
    "glean.error.invalid_label",
    "glean.error.invalid_overflow",
    "glean.error.invalid_state",
    "glean.error.invalid_value",
    /* ping_info fields */
    "glean.internal.metrics.start_time",
    "glean.internal.metrics.end_time",
    "glean.internal.metrics.experiments",
    "glean.internal.metrics.reason",
    "glean.internal.metrics.seq",
    /* hard-coded */
    "glean.internal.metrics.telemetry_sdk_build",
    /* adhoc in src/ping/mod.rs */
    "glean.ping.uploader_capabilities",
    /* adhoc events */
    "glean.restarted",
    "glean.session_end",
    "glean.session_start",
    /* in foreign language wrapper */
    "glean.validation.foreground_count",
];

#[derive(Clone, Default, Debug, Eq, PartialEq)]
struct Metric {
    name: String,
    category: String,
    send_in_pings: Vec<String>,
    lifetime: String,
}

impl Metric {
    fn id(&self) -> String {
        format!("{}.{}", self.category, self.name)
    }
}

/// `prefix "string" suffix`
fn extract_string(line: &str) -> String {
    let mut parts = line.split('"');
    parts
        .next()
        .unwrap_or_else(|| panic!("prefix before string opening missing in:\n{line}"));
    let result = parts
        .next()
        .unwrap_or_else(|| panic!("string in quotes missing in:\n{line}"));
    parts
        .next()
        .unwrap_or_else(|| panic!("suffix after string closing missing in:\n{line}"));
    result.to_string()
}

/// `send_in_pings: vec!["metrics".into(), ...],`
fn extract_array(line: &str) -> Vec<String> {
    let array_start = line
        .find('[')
        .unwrap_or_else(|| panic!("vec![ missing in\n{line}"))
        + 1;
    let array_end = line.rfind(']').expect("array close");
    assert!(array_start < array_end);
    line[array_start..array_end]
        .split(',')
        .map(extract_string)
        .collect()
}

/// `lifetime: Lifetime::Ping`
fn extract_lifetime(line: &str) -> String {
    let start = line
        .find('L')
        .unwrap_or_else(|| panic!("`Lifetime::` missing in\n{line}"));
    let end = line
        .rfind(',')
        .unwrap_or_else(|| panic!(", after Lifetime::* missing in\n{line}"));
    line[start + "Lifetime::".len()..end].to_lowercase()
}

/// Extract definitions in code.
fn extract_metrics_from_code(map: &mut HashMap<String, Metric>, file_path: &str) {
    let src =
        fs::read_to_string(file_path).unwrap_or_else(|_| panic!("unable to read {file_path}"));

    // We look for the line containing the `CommonMetricData {` opening.
    // We then enforce the ordering of additional fields:
    // - name
    // - category
    // - send_in_pings
    // - lifetime
    //
    // `name` before `category` because the majority of the code was already doing it that way.
    let mut lines = src.lines();
    while let Some(line) = lines.next() {
        if line.contains("CommonMetricData ") {
            let mut metric = Metric::default();
            let line = lines.next().unwrap();
            assert!(line.contains("identifier:"));
            let identifier = extract_string(line);
            let (category, name) = identifier
                .rsplit_once('.')
                .unwrap_or_else(|| ("", &identifier));
            metric.category = category.to_string();
            metric.name = name.to_string();

            // Special-casing some internals.
            if metric.category.is_empty() {
                metric.category = String::from("glean.internal.metrics");
            }
            if metric.category == "attribution" {
                metric.category = String::from("glean.internal.metrics.attribution");
            }
            if metric.category == "distribution" {
                metric.category = String::from("glean.internal.metrics.distribution");
            }

            let line = lines.next().unwrap();
            assert!(line.contains("send_in_pings:"));
            metric.send_in_pings = extract_array(line);
            metric.send_in_pings.sort();

            let line = lines.next().unwrap();
            assert!(line.contains("lifetime:"));
            metric.lifetime = extract_lifetime(line);

            let id = metric.id();
            assert!(
                map.insert(metric.id(), metric).is_none(),
                "duplicated metric in code: {id}"
            );
        }
    }
}

/// Extract definitions in YAML.
fn extract_metrics_from_yaml(map: &mut HashMap<String, Metric>, file_path: &str) {
    let metrics_definitions = fs::read_to_string(file_path).expect("unable to read metrics.yaml");
    let docs = YamlLoader::load_from_str(&metrics_definitions).unwrap();
    let docs = &docs[0];

    for (category, metrics) in docs.as_hash().expect("need top-level to be a mapping") {
        let category = category.as_str().unwrap();
        if category == "$schema" {
            continue;
        }

        for (metric_name, metric_definition) in
            metrics.as_hash().expect("metric needs to be hashmap")
        {
            let metric_name = metric_name
                .as_str()
                .expect("metric name needs to be a string");
            let mut metric = Metric {
                category: category.to_string(),
                name: metric_name.to_string(),
                // Defaulting to `ping` lifetime in case the definition doesn't set it
                lifetime: String::from("ping"),
                send_in_pings: vec![],
            };

            for (key, value) in metric_definition
                .as_hash()
                .expect("metric definition needs to be a hashmap")
            {
                let key = key.as_str().unwrap();
                match key {
                    "send_in_pings" => {
                        let send_in_pings =
                            value.as_vec().expect("send_in_pings needs to be a list");
                        metric.send_in_pings = send_in_pings
                            .iter()
                            .map(|ping| {
                                ping.as_str()
                                    .expect("ping in list needs to be a string")
                                    .to_string()
                            })
                            .collect();
                        metric.send_in_pings.sort();
                    }
                    "lifetime" => {
                        let lifetime = value.as_str().expect("lifetime needs to be a string");
                        metric.lifetime = lifetime.to_string();
                    }
                    _ => {}
                }
            }
            assert!(
                map.insert(metric.id(), metric).is_none(),
                "duplicated metric defined"
            );
        }
    }
}

#[test]
fn keep_internal_metrics_in_sync_with_definitions() {
    let mut metrics_in_code = HashMap::new();
    let mut definitions = HashMap::new();

    // Relative to `glean-core`
    extract_metrics_from_code(&mut metrics_in_code, "src/internal_metrics.rs");
    extract_metrics_from_code(&mut metrics_in_code, "src/core_metrics.rs");
    extract_metrics_from_code(&mut metrics_in_code, "src/glean_metrics.rs");

    extract_metrics_from_yaml(&mut definitions, "metrics.yaml");
    extract_metrics_from_yaml(&mut definitions, "android/metrics.yaml");

    // We check in both directions.
    // Only if at least one mismatch is found we fail the test (and provide a useful error message)

    let mut mismatch_found = false;
    let mut msg = String::from("Not all metrics defined in code AND metrics.yaml\n\n");

    msg.push_str("Defined in code, but not in metrics.yaml:\n");
    let mut keys = metrics_in_code.keys().collect::<Vec<_>>();
    keys.sort();
    for key in keys.into_iter() {
        if !definitions.contains_key(key) {
            msg.push_str(&format!("- {key}\n"));
            mismatch_found = true;
        }
    }

    msg.push_str("\nDefined in metrics.yaml, but not in code:\n");
    let mut keys = definitions.keys().collect::<Vec<_>>();
    keys.sort();
    for key in keys.into_iter() {
        if DEFINITION_ONLY.contains(&&key[..]) {
            continue;
        }
        if !metrics_in_code.contains_key(key) {
            msg.push_str(&format!("- {key}\n"));
            mismatch_found = true;
        }
    }

    msg.push_str("\nDefined in DEFINITION_ONLY, but found in code:\n");
    let mut keys = definitions.keys().collect::<Vec<_>>();
    keys.sort();
    for key in keys.into_iter() {
        if !DEFINITION_ONLY.contains(&&key[..]) {
            continue;
        }
        if metrics_in_code.contains_key(key) {
            msg.push_str(&format!("- {key}\n"));
            mismatch_found = true;
        }
    }

    assert!(!mismatch_found, "{msg}");

    // Double-checking lifetime & send_in_pings for code is the same as definition.
    for (id, metric) in metrics_in_code {
        let Some(defined_metric) = definitions.get(&id) else {
            continue;
        };
        assert_eq!(&metric, defined_metric);
    }
}

/// The `server_knobs_config` object metric is a serialized `RemoteSettingsConfig`.
/// Its structure is defined in three places that must agree:
///
/// - `RemoteSettingsConfig` in code
/// - the `structure` of `glean.internal.metrics.server_knobs_config` in `metrics.yaml`
/// - `ping_info.server_knobs_config` in the pipeline schema (`glean.1.schema.json`)
///
/// The schema sets `additionalProperties: false`, so a field added in code but not in
/// the schema causes ingestion errors (see bug 2050579).
#[test]
fn keep_server_knobs_config_in_sync_with_definitions() {
    // No `..Default::default()` here on purpose:
    // adding a field to `RemoteSettingsConfig` must fail to compile until it is added here,
    // which in turn makes this test check it against the definitions below.
    let config = RemoteSettingsConfig {
        metrics_enabled: HashMap::from([("category.name".to_string(), true)]),
        pings_enabled: HashMap::from([("ping".to_string(), true)]),
        event_threshold: Some(41),
        session_sample_rate: Some(1.0),
        events_ping_acceleration_factor: Some(1),
    };
    let serialized = serde_json::to_value(&config).unwrap();
    let in_code: BTreeSet<String> = serialized.as_object().unwrap().keys().cloned().collect();

    // Relative to `glean-core`
    let metrics_definitions =
        fs::read_to_string("metrics.yaml").expect("unable to read metrics.yaml");
    let docs = YamlLoader::load_from_str(&metrics_definitions).unwrap();
    let in_yaml: BTreeSet<String> = docs[0]["glean.internal.metrics"]["server_knobs_config"]
        ["structure"]["properties"]
        .as_hash()
        .expect("server_knobs_config structure properties missing in metrics.yaml")
        .keys()
        .map(|k| k.as_str().unwrap().to_string())
        .collect();

    let schema = fs::read_to_string("../glean.1.schema.json").expect("unable to read schema");
    let schema: serde_json::Value = serde_json::from_str(&schema).unwrap();
    let in_schema: BTreeSet<String> = schema["properties"]["ping_info"]["properties"]
        ["server_knobs_config"]["properties"]
        .as_object()
        .expect("ping_info.server_knobs_config properties missing in glean.1.schema.json")
        .keys()
        .cloned()
        .collect();

    assert_eq!(
        in_code, in_yaml,
        "`RemoteSettingsConfig` fields and `server_knobs_config` structure in metrics.yaml differ"
    );
    assert_eq!(
        in_code, in_schema,
        "`RemoteSettingsConfig` fields and `ping_info.server_knobs_config` in glean.1.schema.json differ.\n\
         Update the schema in mozilla-services/mozilla-pipeline-schemas first, then vendor it with `bin/update-schema.sh <commit>`."
    );
}
