fn main() {
    // Only link llama.cpp if the feature is enabled
    if std::env::var_os("CARGO_FEATURE_NATIVE_LLAMA").is_some() {
        println!("cargo:rerun-if-changed=build.rs");
        let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search=native={}", out_dir.display());
        println!("cargo:rustc-link-lib=static=llama");
    }
} 