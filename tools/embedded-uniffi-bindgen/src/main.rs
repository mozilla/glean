/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::env;

use anyhow::{bail, Context};
use camino::Utf8PathBuf;
use uniffi::TargetLanguage;

fn parse_language(lang: &str) -> anyhow::Result<TargetLanguage> {
    match lang {
        "kotlin" => Ok(TargetLanguage::Kotlin),
        "python" => Ok(TargetLanguage::Python),
        "swift" => Ok(TargetLanguage::Swift),
        _ => bail!("Unknown language"),
    }
}

fn gen_bindings(
    library_file: camino::Utf8PathBuf,
    config_file: Option<camino::Utf8PathBuf>,
    languages: Vec<TargetLanguage>,
    out_dir: camino::Utf8PathBuf,
    crate_name: Option<String>,
) -> anyhow::Result<()> {
    use uniffi::{generate, GenerateOptions};
    let opts = GenerateOptions {
        languages,
        source: library_file,
        out_dir,
        config_override: config_file,
        format: false,
        crate_filter: crate_name,
        metadata_no_deps: false,
    };
    generate(opts)
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

    let Some(library_file) = library_file else {
        bail!("Need path to library file.");
    };

    if target_languages.is_empty() {
        bail!("Need at least one language to generate code for.");
    }

    let Some(out_dir) = out_dir else {
        bail!("Need output directory.")
    };

    gen_bindings(
        library_file,
        config,
        target_languages,
        out_dir,
        Some(String::from("glean_core")),
    )?;

    Ok(())
}
