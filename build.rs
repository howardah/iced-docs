use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=src/content");

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let content_root = manifest_dir.join("src/content");

    let mut files = Vec::new();
    collect_markdown_files(&content_root, &mut files);
    files.sort();

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let out_file = out_dir.join("content_gen.rs");

    let mut generated = String::from(
        "pub struct ContentSource {\n    pub path: &'static str,\n    pub content: &'static str,\n}\n\n",
    );
    generated.push_str("pub static CONTENT_SOURCES: &[ContentSource] = &[\n");

    for file in files {
        let relative = file
            .strip_prefix(&content_root)
            .expect("failed to make content path relative")
            .to_string_lossy()
            .replace('\\', "/");
        let absolute = file.to_string_lossy();
        generated.push_str(&format!(
            "    ContentSource {{ path: \"{}\", content: include_str!(\"{}\") }},\n",
            escape_literal(&relative),
            escape_literal(&absolute)
        ));
    }

    generated.push_str("] ;\n");

    fs::write(out_file, generated).expect("failed to write generated content index");
}

fn collect_markdown_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_files(&path, files);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }
}

fn escape_literal(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
