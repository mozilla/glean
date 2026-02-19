// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::time::Duration;
use std::{env, fs};

use glean::{net, ClientInfoMetrics, ConfigurationBuilder, ErrorType, TestGetValue};

#[allow(clippy::all)] // Don't lint generated code.
pub mod glean_metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

#[derive(Debug)]
struct Uploader;

impl net::PingUploader for Uploader {
    fn upload(&self, _upload_request: net::CapablePingUploadRequest) -> net::UploadResult {
        net::UploadResult::recoverable_failure()
    }
}

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);

    let data_path = args.next().expect("data path required");
    let verify = args.next().map(|arg| arg == "verify").unwrap_or(false);
    if !verify {
        println!("Removing {data_path}");
        _ = fs::remove_dir_all(&data_path);
    }

    let cfg = ConfigurationBuilder::new(true, data_path, "org.mozilla.glean_core.example")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(false)
        .with_uploader(Uploader)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
    };

    _ = &*glean_metrics::prototype;
    glean::initialize(cfg, client_info);

    if verify {
        println!("Verifying metric values...");
        assert!(glean_metrics::test_metrics::sample_boolean
            .test_get_value(None)
            .unwrap());
        assert_eq!(
            Some(1),
            glean_metrics::test_metrics::sample_counter.test_get_value(None)
        );
        assert_eq!(
            "https://example.com",
            glean_metrics::test_metrics::sample_url
                .test_get_value(None)
                .unwrap()
        );
        assert_eq!(
            1,
            glean_metrics::test_metrics::sample_url
                .test_get_num_recorded_errors(ErrorType::InvalidValue)
        );

        assert_eq!(
            1,
            glean_metrics::test_metrics::sample_labeled_counter
                .get("test")
                .test_get_value(None)
                .unwrap()
        );
        assert_eq!(
            "foo",
            glean_metrics::test_metrics::sample_labeled_string
                .get("test")
                .test_get_value(None)
                .unwrap()
        );

        let exp_balloons = serde_json::json!([
            { "colour": "red", "diameter": 5 },
            { "colour": "blue" },
        ]);
        assert_eq!(
            Some(exp_balloons),
            glean_metrics::party::balloons.test_get_value(None)
        );

        let exp_chooser = serde_json::json!([
            { "key": "fortytwo", "value": 42 },
            { "key": "to-be", "value": false },
        ]);
        assert_eq!(
            Some(exp_chooser),
            glean_metrics::party::chooser.test_get_value(None)
        );
        assert_eq!(
            2,
            glean_metrics::test_dual_labeled::static_static
                .get("key1", "category1")
                .test_get_value(None)
                .unwrap()
        );
        assert_eq!(
            0,
            glean_metrics::test_dual_labeled::static_static
                .test_get_num_recorded_errors(ErrorType::InvalidLabel)
        );
        assert_eq!(
            0,
            glean_metrics::test_dual_labeled::dynamic_static
                .test_get_num_recorded_errors(ErrorType::InvalidLabel)
        );
        assert_eq!(
            Some(3),
            glean_metrics::test_dual_labeled::dynamic_static
                .get("party", "category1")
                .test_get_value(None)
        );
        assert_eq!(
            0,
            glean_metrics::test_dual_labeled::static_dynamic
                .test_get_num_recorded_errors(ErrorType::InvalidLabel)
        );
        assert_eq!(
            Some(4),
            glean_metrics::test_dual_labeled::static_dynamic
                .get("key1", "balloons")
                .test_get_value(None)
        );
        assert_eq!(
            0,
            glean_metrics::test_dual_labeled::dynamic_dynamic
                .test_get_num_recorded_errors(ErrorType::InvalidLabel)
        );
        assert_eq!(
            Some(5),
            glean_metrics::test_dual_labeled::dynamic_dynamic
                .get("party", "balloons")
                .test_get_value(None)
        );

        assert_eq!(
            Some(6),
            glean_metrics::test_dual_labeled::static_static
                .get("__other__", "__other__")
                .test_get_value(None)
        );

        assert_eq!(
            0,
            glean_metrics::party::drinks.test_get_num_recorded_errors(ErrorType::InvalidValue)
        );

        let timings = glean_metrics::test_metrics::timings
            .test_get_value(None)
            .unwrap();
        assert_eq!(100, timings.count);
        assert_eq!(1_000_000_000, timings.sum);
        assert_eq!(100, timings.values[&9975792]);

        println!("OK.");
    } else {
        println!("Setting metric values...");
        glean_metrics::test_metrics::sample_boolean.set(true);
        glean_metrics::test_metrics::sample_counter.add(1);
        glean_metrics::test_metrics::sample_url.set("https://example.com");
        glean_metrics::test_metrics::sample_url.set("data:application/json");
        glean_metrics::test_metrics::sample_labeled_counter
            .get("test")
            .add(1);
        glean_metrics::test_metrics::sample_labeled_string
            .get("test")
            .set(String::from("foo"));

        use glean_metrics::party::{BalloonsObject, BalloonsObjectItem};
        let balloons = BalloonsObject::from([
            BalloonsObjectItem {
                colour: Some("red".to_string()),
                diameter: Some(5),
            },
            BalloonsObjectItem {
                colour: Some("blue".to_string()),
                diameter: None,
            },
        ]);
        glean_metrics::party::balloons.set(balloons);

        use glean_metrics::party::{ChooserObject, ChooserObjectItem, ChooserObjectItemValueEnum};
        let mut ch = ChooserObject::new();
        let it = ChooserObjectItem {
            key: Some("fortytwo".to_string()),
            value: Some(ChooserObjectItemValueEnum::Number(42)),
        };
        ch.push(it);
        let it = ChooserObjectItem {
            key: Some("to-be".to_string()),
            value: Some(ChooserObjectItemValueEnum::Boolean(false)),
        };
        ch.push(it);
        glean_metrics::party::chooser.set(ch);

        glean_metrics::test_dual_labeled::static_static
            .get("key1", "category1")
            .add(2);
        glean_metrics::test_dual_labeled::dynamic_static
            .get("party", "category1")
            .add(3);
        glean_metrics::test_dual_labeled::static_dynamic
            .get("key1", "balloons")
            .add(4);
        glean_metrics::test_dual_labeled::dynamic_dynamic
            .get("party", "balloons")
            .add(5);

        // Testing the `__other__` label.
        glean_metrics::test_dual_labeled::static_static
            .get("party", "balloons")
            .add(6);

        // Testing with empty and null values.
        let drinks = serde_json::json!([
            { "name": "lemonade", "ingredients": ["lemon", "water", "sugar"] },
            { "name": "sparkling-water", "ingredients": [] },
            { "name": "still-water", "ingredients": null },
        ]);
        glean_metrics::party::drinks.set_string(drinks.to_string());

        {
            let mut buffer = glean_metrics::test_metrics::timings.start_buffer();

            let mock_duration = Duration::from_millis(10);
            for _ in 0..100 {
                buffer.accumulate(mock_duration.as_millis() as u64);
            }
        }

        // Ensure Glean actually catches up.
        _ = glean_metrics::party::drinks.test_get_value(None);
        println!("OK.");
    }

    glean::shutdown();
}
