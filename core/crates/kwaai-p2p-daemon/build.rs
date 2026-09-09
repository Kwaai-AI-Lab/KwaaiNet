use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn ensure_protoc(out_dir: &Path) {
    // Check if protoc is already in PATH
    if Command::new("protoc").arg("--version").output().is_ok() {
        println!("cargo:warning=Found protoc in PATH");
        return;
    }

    println!("cargo:warning=protoc not found in PATH, downloading...");

    // Detect platform and architecture
    let (platform, archive_ext) = if cfg!(target_os = "windows") {
        ("win64", "zip")
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            ("osx-aarch_64", "zip")
        } else {
            ("osx-x86_64", "zip")
        }
    } else if cfg!(target_os = "linux") {
        if cfg!(target_arch = "aarch64") {
            ("linux-aarch_64", "zip")
        } else {
            ("linux-x86_64", "zip")
        }
    } else {
        eprintln!("\n==========================================================");
        eprintln!("ERROR: Unsupported platform for automatic protoc download");
        eprintln!("==========================================================");
        eprintln!("Please install protoc manually from:");
        eprintln!("  https://github.com/protocolbuffers/protobuf/releases");
        eprintln!("\nOr add it to your PATH.");
        eprintln!("==========================================================\n");
        panic!("Unsupported platform");
    };

    let protoc_dir = out_dir.join("protoc");
    let protoc_bin = if cfg!(windows) {
        protoc_dir.join("bin").join("protoc.exe")
    } else {
        protoc_dir.join("bin").join("protoc")
    };

    if !protoc_bin.exists() {
        println!("cargo:warning=Downloading protoc for {}...", platform);

        std::fs::create_dir_all(&protoc_dir).expect("Failed to create protoc directory");

        let version = "28.3";
        let url = format!(
            "https://github.com/protocolbuffers/protobuf/releases/download/v{}/protoc-{}-{}.{}",
            version, version, platform, archive_ext
        );

        let archive_path = protoc_dir.join(format!("protoc.{}", archive_ext));

        // Download based on platform
        let download_success = if cfg!(windows) {
            // Windows: Use PowerShell
            let download_cmd = format!(
                "Invoke-WebRequest -Uri '{}' -OutFile '{}'",
                url,
                archive_path.display()
            );

            Command::new("powershell")
                .args(["-Command", &download_cmd])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        } else {
            // Linux/macOS: Use curl (more universally available than wget)
            Command::new("curl")
                .args(["-L", "-o"])
                .arg(&archive_path)
                .arg(&url)
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        };

        if !download_success {
            eprintln!("\n==========================================================");
            eprintln!("ERROR: Failed to download protoc!");
            eprintln!("==========================================================");
            eprintln!("Tried downloading from: {}", url);
            eprintln!("\nPlease install protoc manually from:");
            eprintln!("  https://github.com/protocolbuffers/protobuf/releases");
            eprintln!("\nOr add it to your PATH.");
            eprintln!("==========================================================\n");
            panic!("protoc download failed");
        }

        // Extract based on platform
        let extract_success = if cfg!(windows) {
            // Windows: Use PowerShell Expand-Archive
            let extract_cmd = format!(
                "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                archive_path.display(),
                protoc_dir.display()
            );

            Command::new("powershell")
                .args(["-Command", &extract_cmd])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        } else {
            // Linux/macOS: Use unzip
            Command::new("unzip")
                .args(["-o", "-q"]) // -o: overwrite, -q: quiet
                .arg(&archive_path)
                .arg("-d")
                .arg(&protoc_dir)
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        };

        if !extract_success {
            eprintln!("\n==========================================================");
            eprintln!("ERROR: Failed to extract protoc archive!");
            eprintln!("==========================================================");
            if !cfg!(windows) {
                eprintln!("Make sure 'unzip' is installed:");
                eprintln!("  Ubuntu/Debian: sudo apt install unzip");
                eprintln!("  macOS: unzip is pre-installed");
            }
            eprintln!("==========================================================\n");
            panic!("protoc extraction failed");
        }

        // Make protoc executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = std::fs::metadata(&protoc_bin) {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o755); // rwxr-xr-x
                let _ = std::fs::set_permissions(&protoc_bin, permissions);
            }
        }

        println!("cargo:warning=Successfully downloaded and extracted protoc");
    }

    // Set PROTOC environment variable
    env::set_var("PROTOC", &protoc_bin);
    println!("cargo:warning=Using protoc at: {}", protoc_bin.display());
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=proto/p2pd.proto");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let proto_dir = manifest_dir.join("proto");

    ensure_protoc(&out_dir);

    prost_build::Config::new()
        .out_dir(&out_dir)
        .compile_protos(&[proto_dir.join("p2pd.proto")], &[proto_dir])
        .expect("Failed to compile protobuf");
}
