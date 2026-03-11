use std::process::Command;

fn main() {
    let python = find_python();
    let (lib_dir, lib_name) = get_python_lib_info(&python);

    if !lib_dir.is_empty() {
        println!("cargo:rustc-link-search=native={}", lib_dir);
    }
    println!("cargo:rustc-link-lib=dylib={}", lib_name);
}

fn find_python() -> String {
    for candidate in &["python3", "python3.12", "python"] {
        if Command::new(candidate).arg("--version").output().is_ok() {
            return candidate.to_string();
        }
    }
    "python3".to_string()
}

fn get_python_lib_info(python: &str) -> (String, String) {
    let version = Command::new(python)
        .args(["-c", "import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "3.12".to_string());

    let lib_name = format!("python{}", version);

    let prefix = Command::new(python)
        .args(["-c", "import sys; print(sys.prefix)"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    let lib_dir = if cfg!(target_os = "macos") {
        let framework_path = format!(
            "/Library/Frameworks/Python.framework/Versions/{}/lib",
            version
        );
        if std::path::Path::new(&framework_path).exists() {
            framework_path
        } else {
            format!("{}/lib", prefix)
        }
    } else {
        format!("{}/lib", prefix)
    };

    (lib_dir, lib_name)
}