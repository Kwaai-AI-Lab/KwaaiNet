use std::env;
use std::path::PathBuf;
use std::process::Command;

fn ensure_protoc(out_dir: &PathBuf) {
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
                .args(&["-Command", &download_cmd])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        } else {
            // Linux/macOS: Use curl (more universally available than wget)
            Command::new("curl")
                .args(&["-L", "-o"])
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
                .args(&["-Command", &extract_cmd])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        } else {
            // Linux/macOS: Use unzip
            Command::new("unzip")
                .args(&["-o", "-q"])  // -o: overwrite, -q: quiet
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
                permissions.set_mode(0o755);  // rwxr-xr-x
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

    // 1. Check if Go is installed
    let go_version = Command::new("go")
        .arg("version")
        .output();

    match go_version {
        Ok(output) => {
            let version_str = String::from_utf8_lossy(&output.stdout);
            println!("cargo:warning=Found Go: {}", version_str.trim());
        }
        Err(_) => {
            eprintln!("\n==========================================================");
            eprintln!("ERROR: Go toolchain not found!");
            eprintln!("==========================================================");
            eprintln!("kwaai-p2p-daemon requires Go 1.13+ to build the p2pd daemon.");
            eprintln!("\nTo install Go:");
            eprintln!("  Windows: https://golang.org/dl/");
            eprintln!("  Linux:   sudo apt install golang-go  (or your package manager)");
            eprintln!("  macOS:   brew install go");
            eprintln!("\nAfter installing Go, run 'cargo build' again.");
            eprintln!("==========================================================\n");
            panic!("Go toolchain is required to build kwaai-p2p-daemon");
        }
    }

    // 2. Setup paths
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let repo_dir = out_dir.join("go-libp2p-daemon");

    // Use profile directory for the daemon binary (easier to find and use)
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target")
        .join(&profile);

    let daemon_binary = if cfg!(windows) {
        target_dir.join("p2pd.exe")
    } else {
        target_dir.join("p2pd")
    };

    // 3. Clone go-libp2p-daemon if not exists
    if !repo_dir.exists() {
        println!("cargo:warning=Cloning go-libp2p-daemon repository...");

        let clone_status = Command::new("git")
            .args(&[
                "clone",
                "--depth", "1",
                "--branch", "v0.5.0.hivemind1",
                "https://github.com/learning-at-home/go-libp2p-daemon.git",
            ])
            .arg(&repo_dir)
            .status();

        match clone_status {
            Ok(status) if status.success() => {
                println!("cargo:warning=Successfully cloned go-libp2p-daemon");
            }
            Ok(status) => {
                eprintln!("Failed to clone go-libp2p-daemon: exit code {:?}", status.code());
                panic!("Git clone failed");
            }
            Err(e) => {
                eprintln!("Failed to execute git: {}", e);
                eprintln!("Make sure git is installed and in your PATH");
                panic!("Git not found");
            }
        }
    } else {
        println!("cargo:warning=Using existing go-libp2p-daemon repository");
    }

    // 4. Build the daemon
    println!("cargo:warning=Building p2pd daemon from source...");

    let build_status = Command::new("go")
        .args(&["build", "-o"])
        .arg(&daemon_binary)
        .arg("./p2pd")
        .current_dir(&repo_dir)
        .status();

    match build_status {
        Ok(status) if status.success() => {
            println!("cargo:warning=Successfully built p2pd daemon at: {}", daemon_binary.display());
        }
        Ok(status) => {
            eprintln!("Failed to build p2pd: exit code {:?}", status.code());
            panic!("Go build failed");
        }
        Err(e) => {
            eprintln!("Failed to execute go build: {}", e);
            panic!("Go build execution failed");
        }
    }

    // 5. Copy p2pd.proto if not already present
    let proto_src = repo_dir.join("pb").join("p2pd.proto");
    let proto_dst = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("proto")
        .join("p2pd.proto");

    if proto_src.exists() && !proto_dst.exists() {
        std::fs::copy(&proto_src, &proto_dst)
            .expect("Failed to copy p2pd.proto");
        println!("cargo:warning=Copied p2pd.proto to proto/");
    }

    // 6. Ensure protoc is available
    ensure_protoc(&out_dir);

    // 7. Generate Rust protobuf code
    if proto_dst.exists() {
        println!("cargo:warning=Generating Rust code from p2pd.proto...");

        prost_build::Config::new()
            .out_dir(&out_dir)
            .compile_protos(&[proto_dst], &[PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("proto")])
            .expect("Failed to compile protobuf");

        println!("cargo:warning=Successfully generated protobuf Rust code");
    }

    // 7. Set environment variable for runtime daemon path
    println!("cargo:rustc-env=P2PD_PATH={}", daemon_binary.display());
    println!("cargo:rustc-env=P2PD_REPO={}", repo_dir.display());
}
