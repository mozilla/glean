/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::env;

use anyhow::{bail, Context};
use camino::Utf8PathBuf;
use glob::glob;

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

fn find_library_file(crate_root: &camino::Utf8Path, library_name: &str) -> Utf8PathBuf {
    let path = if let Ok(target_dir) = env::var("CARGO_TARGET_DIR") {
        Utf8PathBuf::from(target_dir)
    } else {
        crate_root.join("target")
    };

    // Search for all viable patterns and pick the first one we can find.
    let glob_patterns = [
        format!("{path}/**/lib{library_name}.a"),
        format!("{path}/**/lib{library_name}.so"),
        format!("{path}/**/lib{library_name}.dylib"),
        format!("{path}/**/{library_name}.dll"),
    ];

    for pattern in &glob_patterns {
        if let Some(Ok(path)) = glob(pattern).unwrap().next() {
            return Utf8PathBuf::from_path_buf(path).unwrap();
        }
    }

    panic!("lib{library_name} could not be found in {path}")
}

fn gen_bindings(
    udl_file: &camino::Utf8Path,
    config_file: Option<&camino::Utf8Path>,
    languages: Vec<TargetLanguage>,
    out_dir: Option<&camino::Utf8Path>,
    crate_name: Option<&str>,
) -> anyhow::Result<()> {
    use uniffi::generate_bindings;
    use uniffi::{KotlinBindingGenerator, PythonBindingGenerator, SwiftBindingGenerator};

    // 3 parents
    // Should be `glean-core/src/glean.udl`,
    // the one in `glean-core/bundle` is a symlink.
    let canonical = udl_file.canonicalize().unwrap();
    let crate_root = camino::Utf8Path::from_path(
        canonical
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap(),
    )
    .unwrap();

    for language in languages {
        match language {
            TargetLanguage::Kotlin => {
                let library_file = find_library_file(crate_root, "xul");
                generate_bindings(
                    udl_file,
                    config_file,
                    KotlinBindingGenerator,
                    out_dir,
                    Some(&library_file),
                    crate_name,
                    false,
                )?
            }
            TargetLanguage::Python => {
                let library_file = find_library_file(crate_root, "glean_ffi");
                generate_bindings(
                    udl_file,
                    config_file,
                    PythonBindingGenerator,
                    out_dir,
                    Some(&library_file),
                    crate_name,
                    false,
                )?
            }
            TargetLanguage::Swift => {
                let library_file = find_library_file(crate_root, "glean_ffi");
                generate_bindings(
                    udl_file,
                    config_file,
                    SwiftBindingGenerator,
                    out_dir,
                    Some(&library_file),
                    crate_name,
                    false,
                )?
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
        bail!("Need UDL file.");
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
        Some("glean_core"),
    )?;

    Ok(())
}
