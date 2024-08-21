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
    udl_file: &camino::Utf8Path,
    cfo: Option<&camino::Utf8Path>,
    languages: Vec<TargetLanguage>,
    odo: Option<&camino::Utf8Path>,
    library_file: Option<&camino::Utf8Path>,
    crate_name: Option<&str>,
    fmt: bool,
) -> anyhow::Result<()> {
    use uniffi::generate_bindings;
    use uniffi::{KotlinBindingGenerator, PythonBindingGenerator, SwiftBindingGenerator};

    for language in languages {
        match language {
            TargetLanguage::Kotlin => generate_bindings(
                udl_file,
                cfo,
                KotlinBindingGenerator,
                odo,
                library_file,
                crate_name,
                fmt,
            )?,
            TargetLanguage::Python => generate_bindings(
                udl_file,
                cfo,
                PythonBindingGenerator,
                odo,
                library_file,
                crate_name,
                fmt,
            )?,
            TargetLanguage::Swift => generate_bindings(
                udl_file,
                cfo,
                SwiftBindingGenerator,
                odo,
                library_file,
                crate_name,
                fmt,
            )?,
        };
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut args = env::args().skip(1);

    if args.next().as_deref() != Some("generate") {
        bail!("Only the `generate` subcommand is supported.");
    }

    let mut udl_file = None;
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
                _ => bail!("Unsupported option: {arg}"),
            }
        } else if udl_file.is_some() {
            bail!("UDL file already set.");
        } else {
            udl_file = Some(Utf8PathBuf::from(arg));
        }
    }

    let out_dir = out_dir.map(Utf8PathBuf::from);
    let config = config.map(Utf8PathBuf::from);

    if udl_file.is_none() {
        bail!("Need UDL file");
    }

    if target_languages.is_empty() {
        bail!("Need at least one language to generate code for.");
    }

    if out_dir.is_none() {
        bail!("Need output directory.")
    }

    gen_bindings(
        &udl_file.unwrap(),
        config.as_deref(),
        target_languages,
        out_dir.as_deref(),
        None,
        Some("glean_core"),
        false,
    )?;

    Ok(())
}
