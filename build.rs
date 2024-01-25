fn main() {
    if cfg!(target_os = "macos") {
        pyo3_build_config::add_extension_module_link_args();
        println!(
            "cargo:rustc-link-search=native=/opt/homebrew/Cellar/python@3.11/3.11.6/Frameworks/Python.framework/Versions/3.11/lib"
        );
        println!("cargo:rustc-link-lib=dylib=python3.11");
    } else {
        println!("cargo:rustc-link-lib=dylib=python3.11")
    }
}
