fn main() {
    if cfg!(target_os = "macos") {
        pyo3_build_config::add_extension_module_link_args();
        println!(
            "cargo:rustc-link-search=native=/Library/Frameworks/Python.framework/Versions/3.12/lib"
        );
        println!("cargo:rustc-link-lib=dylib=python3.12");
    } else {
        println!("cargo:rustc-link-lib=dylib=python3.12")
    }
}
