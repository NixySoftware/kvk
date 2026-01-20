use std::{fs, path::Path};

use serde_norway::Value;

const PACKAGE: &str = "kvk";

fn main() {
    let mut specifications = vec![];
    let mut packages = vec![];

    for entry in fs::read_dir(Path::new("/tmp/kvk")).expect("Directory should be read.") {
        let entry = entry.expect("Entry should exist.");
        let path = entry.path();

        if !path.is_file() || path.extension().is_none_or(|extension| extension != "yaml") {
            continue;
        }

        let feature = path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .expect("File stem should be a string.");
        let package = feature.replace("-", "_");

        let specification = serde_norway::from_str::<Value>(
            &fs::read_to_string(&path).expect("Specification should be read."),
        )
        .expect("Specification should be valid YAML.");

        let section = specification
            .get("info")
            .expect("Specification should have info section.")
            .as_mapping()
            .expect("Section should be a mapping.");

        let title = section
            .get("title")
            .expect("Info should have title.")
            .as_str()
            .expect("Title should be a string.")
            .replace("API ", "")
            .replace("Dataset ", "Data ")
            .replace("HP Mutatie Service - Delivery", "Mutatieservice")
            .replace("KVK Handelsregister ", "");
        let version = section
            .get("version")
            .expect("Info should have version.")
            .as_str()
            .expect("Version should be a string.")
            .to_owned();

        specifications.push((title, version));
        packages.push((feature.to_owned(), package));
    }

    specifications.sort_by_key(|(title, _)| title.clone());
    packages.sort();

    let comments = specifications
        .iter()
        .map(|(title, version)| format!("//! - {title} - {version}"))
        .collect::<Vec<_>>()
        .join("\n");
    let packages = packages
        .iter()
        .map(|(feature, package)| format!("#[cfg(feature = \"{feature}\")]\npub mod {package};"))
        .collect::<Vec<_>>()
        .join("\n");

    let contents = format!(
        "//! Rust OpenAPI client for [Kamer van Koophandel](https://www.kvk.nl/).\n//!\n//! For more information see <https://developers.kvk.nl/>.\n//!\n//! ## OpenAPI specification versions:\n{comments}\n\n{packages}\n"
    );

    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Directory should have parent.")
        .join("packages")
        .join(PACKAGE)
        .join("src")
        .join("lib.rs");

    fs::write(path, contents).expect("Code should be written.");
}
