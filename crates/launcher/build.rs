use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=../../web-shell");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR must be set by Cargo");
    let embedded_dir = Path::new(&out_dir).join("embedded-web-shell");
    fs::create_dir_all(&embedded_dir).expect("failed to create embedded-web-shell dir");

    let web_shell_src = Path::new("../../web-shell");
    if web_shell_src.exists() && web_shell_src.is_dir() {
        for entry in fs::read_dir(web_shell_src).expect("failed to read web-shell directory") {
            let entry = entry.expect("invalid dir entry");
            let path = entry.path();
            if path.is_file() {
                let dest = embedded_dir.join(entry.file_name());
                fs::copy(&path, &dest).expect("failed to copy web-shell asset");
            }
        }
    }

    let index_path = embedded_dir.join("index.html");
    if !index_path.exists() {
        fs::write(
            &index_path,
            "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><title>~ L@u’s Skill Labs</title></head><body><h1>~ L@u’s Skill Labs Embedded Shell</h1></body></html>",
        )
        .expect("failed to write fallback index.html");
    }

    let manifest_path = embedded_dir.join("manifest.json");
    if !manifest_path.exists() {
        fs::write(
            &manifest_path,
            r#"{"name":"~ L@u’s Skill Labs","short_name":"~L@u Labs","display":"standalone","start_url":"/index.html"}"#,
        )
        .expect("failed to write fallback manifest.json");
    }

    let sw_path = embedded_dir.join("sw.js");
    if !sw_path.exists() {
        fs::write(
            &sw_path,
            "// Cache-First Service Worker fallback for ~ L@u's Skill Labs\n",
        )
        .expect("failed to write fallback sw.js");
    }
}
