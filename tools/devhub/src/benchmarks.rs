use serde::Deserialize;
use xshell::{Shell, cmd};

use super::{Metric, MetricRecorder, Result};

/// Extract the listed metrics out from all benchmarks, grouped by the benchmark name.
const JQ_SCRIPT: &str = r#"
  .profiles[0].summaries.parts[0].metrics_summary.Cachegrind as $cachegrind
| .function_name + " " + .details as $name
| [ "Ir", "EstimatedCycles", "TotalRW", "L1hits", "LLhits", "RamHits" ] as $keys
| $keys
| map({
  name: $name + " -- " + .,
  value: $cachegrind[.].metrics.Both[0].Int
})
"#;

/// Parse Gungraun benchmark results
///
/// Expects a `gungraun-output.json` file in the current directory.
pub struct Benchmark;

impl MetricRecorder for Benchmark {
    fn name(&self) -> &'static str {
        "benchmark"
    }

    fn description(&self) -> &'static str {
        "Gungraun Benchmark Results"
    }

    fn record(&self, sh: &Shell) -> Result<Vec<Metric>> {
        let temp = sh.create_temp_dir()?;

        let jq_script_path = temp.path().join("transform-gungraun.jq");
        sh.write_file(&jq_script_path, JQ_SCRIPT)?;

        let input = sh.current_dir().join("gungraun-output.json");

        let mut metrics = Vec::new();

        let benchmarks = cmd!(sh, "jq -cf {jq_script_path} {input}").read()?;
        let benchmarks = benchmarks.lines();
        for line in benchmarks {
            let bench: Vec<Bench> = serde_json::from_str(line)?;

            for b in bench {
                metrics.push(Metric {
                    name: b.name,
                    value: b.value,
                })
            }
        }

        Ok(metrics)
    }
}

#[derive(Deserialize)]
struct Bench {
    name: String,
    value: u64,
}
