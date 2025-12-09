/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::env;

use anyhow::{bail, Context};
use camino::Utf8PathBuf;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum TargetLanguage {
    Kotlin,
    Swift,
    Python,
}

fn parse_language(lang: &str) -> anyhow::Result<TargetLanguage> {
    match lang {
        "kotlin" => Ok(TargetLanguage::Kotlin),
        "python" => Ok(TargetLanguage::Python),
        "swift" => Ok(TargetLanguage::Swift),
        _ => bail!("Unknown language"),
    }
}

fn gen_bindings(
    library_file: &camino::Utf8Path,
    config_file: Option<&camino::Utf8Path>,
    languages: Vec<TargetLanguage>,
    out_dir: &camino::Utf8Path,
    crate_name: Option<String>,
) -> anyhow::Result<()> {
    use uniffi::generate_bindings_library_mode;
    use uniffi::{KotlinBindingGenerator, PythonBindingGenerator, SwiftBindingGenerator};

    use uniffi::CargoMetadataConfigSupplier;
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.no_deps();
    let metadata = cmd.exec().context("error running cargo metadata")?;
    let config_supplier = CargoMetadataConfigSupplier::from(metadata);

    for language in languages {
        match language {
            TargetLanguage::Kotlin => {
                generate_bindings_library_mode(
                    library_file,
                    crate_name.clone(),
                    &KotlinBindingGenerator,
                    &config_supplier,
                    config_file,
                    out_dir,
                    false,
                )?;
            }
            TargetLanguage::Python => {
                generate_bindings_library_mode(
                    library_file,
                    crate_name.clone(),
                    &PythonBindingGenerator,
                    &config_supplier,
                    config_file,
                    out_dir,
                    false,
                )?;
            }
            TargetLanguage::Swift => {
                generate_bindings_library_mode(
                    library_file,
                    crate_name.clone(),
                    &SwiftBindingGenerator,
                    &config_supplier,
                    config_file,
                    out_dir,
                    false,
                )?;
            }
        };
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut args = env::args().skip(1);

    if args.next().as_deref() != Some("generate") {
        bail!("Only the `generate` subcommand is supported.");
    }

    let mut library_file = None;
    let mut target_languages = vec![];
    let mut out_dir = None;
    let mut config = None;

    while let Some(arg) = args.next() {
        if let Some(arg) = arg.strip_prefix("--") {
            match arg {
                "language" => {
                    let lang = args.next().context("--language needs a parameter")?;
                    let lang = parse_language(&lang)?;
                    target_languages.push(lang);
                }
                "out-dir" => out_dir = Some(args.next().context("--out-dir needs a parameter")?),
                "no-format" => {
                    // this is the default anyway.
                }
                "config" => {
                    config = Some(args.next().context("--config needs a parameter")?);
                }
                "library" => {
                    library_file = Some(args.next().context("--library needs a parameter")?);
                }
                _ => bail!("Unsupported option: {arg}"),
            }
        } else {
            bail!("Unknown parameter {arg:?}");
        }
    }

    let library_file = library_file.map(Utf8PathBuf::from);
    let out_dir = out_dir.map(Utf8PathBuf::from);
    let config = config.map(Utf8PathBuf::from);

    if library_file.is_none() {
        bail!("Need path to library file.");
    }

    if target_languages.is_empty() {
        bail!("Need at least one language to generate code for.");
    }

    if out_dir.is_none() {
        bail!("Need output directory.")
    }

    gen_bindings(
        &library_file.unwrap(),
        config.as_deref(),
        target_languages,
        &out_dir.unwrap(),
        Some(String::from("glean_core")),
    )?;

    Ok(())
}
