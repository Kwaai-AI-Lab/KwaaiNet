//! Compile kwaai.proto into both server and client stubs via tonic-build.
//!
//! Mirrors the protoc-bootstrap approach used by kwaai-p2p-daemon's build.rs
//! so a developer without a system protoc still gets a working build: if
//! protoc isn't on PATH, we download a pinned release into OUT_DIR/protoc and
//! point tonic-build at that binary via $PROTOC.

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn ensure_protoc(out_dir: &Path) {
    if Command::new("protoc").arg("--version").output().is_ok() {
        return;
    }

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
        panic!(
            "kwaai-rpc build: unsupported platform for automatic protoc download. \
             Install protoc manually and re-run cargo build."
        );
    };

    let protoc_dir = out_dir.join("protoc");
    let protoc_bin = if cfg!(windows) {
        protoc_dir.join("bin").join("protoc.exe")
    } else {
        protoc_dir.join("bin").join("protoc")
    };

    if !protoc_bin.exists() {
        std::fs::create_dir_all(&protoc_dir).expect("create protoc dir");

        let version = "28.3";
        let url = format!(
            "https://github.com/protocolbuffers/protobuf/releases/download/v{}/protoc-{}-{}.{}",
            version, version, platform, archive_ext
        );
        let archive_path = protoc_dir.join(format!("protoc.{}", archive_ext));

        let download_ok = if cfg!(windows) {
            let cmd = format!(
                "Invoke-WebRequest -Uri '{}' -OutFile '{}'",
                url,
                archive_path.display()
            );
            Command::new("powershell")
                .args(["-Command", &cmd])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        } else {
            Command::new("curl")
                .args(["-L", "-o"])
                .arg(&archive_path)
                .arg(&url)
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        };
        if !download_ok {
            panic!("kwaai-rpc build: failed to download protoc from {}", url);
        }

        let extract_ok = if cfg!(windows) {
            let cmd = format!(
                "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                archive_path.display(),
                protoc_dir.display()
            );
            Command::new("powershell")
                .args(["-Command", &cmd])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        } else {
            Command::new("unzip")
                .args(["-o", "-q"])
                .arg(&archive_path)
                .arg("-d")
                .arg(&protoc_dir)
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        };
        if !extract_ok {
            panic!("kwaai-rpc build: failed to extract protoc archive");
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = std::fs::metadata(&protoc_bin) {
                let mut perms = meta.permissions();
                perms.set_mode(0o755);
                let _ = std::fs::set_permissions(&protoc_bin, perms);
            }
        }
    }

    env::set_var("PROTOC", &protoc_bin);
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=proto/kwaai.proto");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    ensure_protoc(&out_dir);

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let proto_dir = manifest_dir.join("proto");
    let proto = proto_dir.join("kwaai.proto");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&[proto], &[proto_dir])
        .expect("compile kwaai.proto");
}
