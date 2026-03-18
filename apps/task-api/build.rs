use std::{fs, path::PathBuf};

fn main() {
    let (_, api) = api::openapi::router();

    let json = serde_json::to_string_pretty(&api).unwrap();

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/openapi.json");

    fs::write(root, json).unwrap();
}
