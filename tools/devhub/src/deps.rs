use std::collections::HashSet;

use xshell::{Shell, cmd};

use super::{Metric, MetricRecorder, Result};

/// Number of dependencies.
pub struct Dependencies(String);

impl Dependencies {
    pub fn new<S: Into<String>>(krate: S) -> Dependencies {
        Dependencies(krate.into())
    }
}

impl MetricRecorder for Dependencies {
    fn name(&self) -> &'static str {
        "deps"
    }

    fn description(&self) -> &'static str {
        "Number of dependencies"
    }

    fn record(&self, sh: &Shell) -> Result<Vec<Metric>> {
        let mut metrics = Vec::new();

        let krate = &self.0;
        let dep_count = cmd!(sh, "cargo tree --locked --offline -p {krate} --edges no-dev --charset ascii --prefix none --format '{p}'").read()?;
        let dep_count = dep_count.lines().collect::<HashSet<_>>().len() as u64;
        metrics.push(Metric {
            name: format!("Number of dependencies for {}", self.0),
            value: dep_count,
        });

        Ok(metrics)
    }
}
