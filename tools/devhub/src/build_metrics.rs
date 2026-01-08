use std::collections::HashMap;

use serde::Deserialize;
use xshell::{Shell, cmd};

use super::{Metric, MetricRecorder, Result};

type Tokei = HashMap<String, TokeiStats>;

#[derive(Deserialize)]
struct TokeiStats {
    code: u64,
}

/// Count the lines of code.
pub struct Codesize;

impl MetricRecorder for Codesize {
    fn name(&self) -> &'static str {
        "loc"
    }

    fn description(&self) -> &'static str {
        "Lines of Code (by language)"
    }

    fn record(&self, sh: &Shell) -> Result<Vec<Metric>> {
        let tokei_out = cmd!(
            sh,
            "tokei -o json -t rust,kotlin,swift,python glean-core"
        )
        .read()?;
        let tokei_json: Tokei = serde_json::from_str(&tokei_out)?;

        let mut metrics = Vec::new();
        for (lang, stats) in tokei_json {
            let metric = Metric {
                name: format!("Lines of code - {lang}"),
                value: stats.code,
            };
            metrics.push(metric);
        }

        Ok(metrics)
    }
}

/// Count the number of built-in metrics
pub struct MetricCount;

impl MetricRecorder for MetricCount {
    fn name(&self) -> &'static str {
        "metrics"
    }

    fn description(&self) -> &'static str {
        "Number of built-in metrics"
    }

    fn record(&self, sh: &Shell) -> Result<Vec<Metric>> {
        let metric_count = cmd!(sh, "rg -c '^  [a-z]' glean-core/metrics.yaml")
            .read()?
            .parse()?;
        let metric = Metric {
            name: String::from("Number of built-in metrics"),
            value: metric_count,
        };
        Ok(vec![metric])
    }
}
