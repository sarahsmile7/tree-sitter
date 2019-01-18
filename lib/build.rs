extern crate cc;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=TREE_SITTER_STATIC_ANALYSIS");
    if env::var("TREE_SITTER_STATIC_ANALYSIS").is_ok() {
        if let (Some(clang_path), Some(scan_build_path)) = (which("clang"), which("scan-build")) {
            let clang_path = clang_path.to_str().unwrap();
            let scan_build_path = scan_build_path.to_str().unwrap();
            env::set_var(
                "CC",
                &format!(
                    "{} -analyze-headers --use-analyzer={} cc",
                    scan_build_path, clang_path
                ),
            );
        }
    }

    let mut config = cc::Build::new();
    config
        .define("UTF8PROC_STATIC", "")
        .flag_if_supported("-std=c99")
        .flag_if_supported("-Wno-unused-parameter")
        .include("include")
        .include("utf8proc")
        .file(Path::new("src").join("lib.c"))
        .compile("tree-sitter");
}

fn which(exe_name: impl AsRef<Path>) -> Option<PathBuf> {
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(&exe_name);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        })
    })
}