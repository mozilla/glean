//! Glean Rust build utility
//!
//! `glean-build` generates the Rust bindings based on metrics and ping definitions,
//! using [`glean-parser`] under the hood.
//!
//! ## Requirements
//!
//! * Python 3
//! * pip
//!
//! ## Usage
//!
//! In your application add a `build.rs` file next to your `Cargo.toml`,
//! then add this code:
//!
//! ```rust,ignore
//! use glean_build::Builder;
//!
//! fn main() {
//!     Builder::default()
//!         .file("metrics.yaml")
//!         .generate()
//!         .expect("Error generating Glean Rust bindings");
//! }
//! ```
//!
//! In your code add the following to include the generated code:
//!
//! ```rust,ignore
//! mod metrics {
//!     include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
//! }
//! ```
//!
//! You can then access your metrics and pings directly by name within the `metrics` module.
//!
//! [`glean-parser`]: https://github.com/mozilla/glean_parser/
use std::env;

use xshell_venv::{Result, Shell, VirtualEnv};

const GLEAN_PARSER_VERSION: &str = "7.0.0";

/// A Glean Rust bindings generator.
pub struct Builder {
    files: Vec<String>,
    out_dir: String,
}

impl Default for Builder {
    /// A default Glean Rust bindings generator.
    ///
    /// Use [`file`][`Builder::file`] and [`files`][`Builder::files`]
    /// to specify the input files.
    fn default() -> Self {
        let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| "".into());
        Self {
            files: vec![],
            out_dir,
        }
    }
}

impl Builder {
    /// A Glean Rust bindings generator with the given output directory.
    ///
    /// Use [`file`][`Builder::file`] and [`files`][`Builder::files`]
    /// to specify the input files.
    pub fn with_output<S: Into<String>>(out_dir: S) -> Self {
        Self {
            files: vec![],
            out_dir: out_dir.into(),
        }
    }

    /// Add a definition file, e.g. `metrics.yaml` or `pings.yaml`.
    ///
    /// Can be called multiple times to add more files.
    pub fn file<S: Into<String>>(&mut self, file: S) -> &mut Self {
        self.files.push(file.into());
        self
    }

    /// Add multiple definition files, e.g. `metrics.yaml` or `pings.yaml`.
    pub fn files<P>(&mut self, files: P) -> &mut Self
    where
        P: IntoIterator,
        P::Item: Into<String>,
    {
        for file in files.into_iter() {
            self.file(file);
        }
        self
    }

    /// Generate the Rust bindings.
    ///
    /// The consumer must include the generated `glean_metrics.rs` file, e.g.:
    ///
    /// ```rust,ignore
    /// include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
    /// ```
    pub fn generate(&self) -> Result<()> {
        let out_dir = &self.out_dir;
        if out_dir.is_empty() {
            panic!("Could not determine output directory.")
        }

        let sh = Shell::new()?;
        let venv = VirtualEnv::new(&sh, "py3-glean_parser")?;

        let glean_parser = format!("glean_parser~={GLEAN_PARSER_VERSION}");
        venv.pip_install(&glean_parser)?;

        for file in &self.files {
            println!("cargo:rerun-if-changed={file}");
        }

        let mut args = vec!["translate", "--format", "rust", "--output", out_dir];
        args.extend(self.files.iter().map(|s| s.as_str()));
        venv.run_module("glean_parser", &args)?;

        Ok(())
    }
}
