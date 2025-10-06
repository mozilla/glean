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
use std::{env, path::PathBuf};

use xshell_venv::{Result, Shell, VirtualEnv};

const GLEAN_PARSER_VERSION: &str = "18.0.1";

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
        let venv = if let Ok(env_dir) = env::var("GLEAN_PYTHON_VENV_DIR") {
            eprintln!("got env dir: {env_dir}");
            let env_path = PathBuf::from(env_dir);
            VirtualEnv::with_path(&sh, &env_path)?
        } else {
            let venv = VirtualEnv::new(&sh, "py3-glean_parser")?;

            let glean_parser = format!("glean_parser~={GLEAN_PARSER_VERSION}");
            // TODO: Remove after we switched glean_parser away from legacy setup.py
            venv.pip_install("setuptools")?;
            venv.pip_install(&glean_parser)?;
            venv
        };

        for file in &self.files {
            println!("cargo:rerun-if-changed={file}");
        }

        let mut args = vec!["translate", "--format", "rust", "--output", out_dir];
        args.extend(self.files.iter().map(|s| s.as_str()));
        venv.run_module("glean_parser", &args)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::{env, fs, path::PathBuf};

    use super::*;

    #[test]
    fn test_builder() {
        // We know this package's location, and it's up 2 folders from there.
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let package_root = manifest_dir.parent().unwrap().parent().unwrap();
        // Ensure xshell-venv create the venv in the expected directory.
        env::set_var(
            "CARGO_TARGET_DIR",
            package_root.join("target").display().to_string(),
        );

        // clean out the previous venv, if it exists
        let venv_dir = package_root.join("target").join("venv-py3-glean_parser");
        if venv_dir.exists() {
            fs::remove_dir_all(venv_dir).unwrap();
        }
        let metrics_yaml = package_root
            .join("samples")
            .join("rust")
            .join("metrics.yaml");
        let out_dir = tempfile::tempdir().unwrap();
        Builder::with_output(out_dir.path().to_string_lossy())
            .file(metrics_yaml.to_string_lossy())
            .generate()
            .expect("Error generating Glean bindings");
    }
}
