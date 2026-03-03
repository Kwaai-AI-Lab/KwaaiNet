//! `kwaainet setup` — initial configuration and dependency installation.

use anyhow::{bail, Context, Result};

use crate::display::{print_box_header, print_info, print_separator, print_success, print_warning};

/// Download and install `p2pd` next to the `kwaainet` binary if it is missing.
pub async fn get_dependencies() -> Result<()> {
    print_box_header("📦 KwaaiNet — Get Dependencies");

    let exe = std::env::current_exe().context("cannot determine current executable path")?;
    let install_dir = exe.parent().context("cannot determine install directory")?;

    #[cfg(windows)]
    let p2pd_name = "p2pd.exe";
    #[cfg(not(windows))]
    let p2pd_name = "p2pd";

    let p2pd_dst = install_dir.join(p2pd_name);

    // Already present next to kwaainet?
    if p2pd_dst.exists() {
        print_success(&format!("p2pd already present at {}", p2pd_dst.display()));
        print_separator();
        return Ok(());
    }

    // Present somewhere on PATH?
    if let Some(path) = find_in_path(p2pd_name) {
        print_success(&format!("p2pd found on PATH at {}", path.display()));
        print_separator();
        return Ok(());
    }

    print_info("p2pd not found — downloading from latest release…");

    // Build the target triple from runtime constants so this works on all platforms
    // without requiring build.rs changes or extra dependencies.
    let target = build_target_triple();
    if target.is_none() {
        print_warning("Unsupported platform — please download p2pd manually from:");
        println!("  https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest");
        print_separator();
        return Ok(());
    }
    let target = target.unwrap();

    #[cfg(windows)]
    let (archive_ext, is_zip) = ("zip", true);
    #[cfg(not(windows))]
    let (archive_ext, is_zip) = ("tar.xz", false);

    let url = format!(
        "https://github.com/Kwaai-AI-Lab/KwaaiNet/releases/latest/download/kwaainet-{}.{}",
        target, archive_ext
    );

    println!("  Target:  {}", target);
    println!("  Archive: kwaainet-{}.{}", target, archive_ext);
    println!();

    // Download
    let client = reqwest::Client::builder()
        .user_agent("kwaainet-setup")
        .build()?;
    let response = client
        .get(&url)
        .send()
        .await
        .context("failed to download archive")?;

    if !response.status().is_success() {
        bail!("download failed: HTTP {}", response.status());
    }

    let bytes = response
        .bytes()
        .await
        .context("failed to read archive bytes")?;

    let archive_size_mb = bytes.len() as f64 / 1_048_576.0;
    println!("  Downloaded {:.1} MB", archive_size_mb);

    // Write to a temp file
    let tmp_dir = std::env::temp_dir().join("kwaainet-setup");
    std::fs::create_dir_all(&tmp_dir).context("failed to create temp dir")?;
    let archive_path = tmp_dir.join(format!("kwaainet-{}.{}", target, archive_ext));
    std::fs::write(&archive_path, &bytes).context("failed to write archive to temp dir")?;

    // Extract p2pd
    let p2pd_src = if is_zip {
        extract_from_zip(&archive_path, &tmp_dir, p2pd_name)?
    } else {
        extract_from_tarxz(&archive_path, &tmp_dir, target, p2pd_name)?
    };

    // Copy to install dir
    std::fs::copy(&p2pd_src, &p2pd_dst)
        .with_context(|| format!("failed to install p2pd to {}", p2pd_dst.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&p2pd_dst, std::fs::Permissions::from_mode(0o755))
            .context("failed to set p2pd executable bit")?;
    }

    // Clean up temp files
    let _ = std::fs::remove_file(&archive_path);
    let _ = std::fs::remove_file(&p2pd_src);

    print_success(&format!("p2pd installed to {}", p2pd_dst.display()));
    print_info("You can now run: kwaainet start --daemon");
    print_separator();
    Ok(())
}

/// Extract `p2pd` from a `.tar.xz` archive using the system `tar` command.
fn extract_from_tarxz(
    archive: &std::path::Path,
    out_dir: &std::path::Path,
    target: &str,
    binary_name: &str,
) -> Result<std::path::PathBuf> {
    // Archive structure: kwaainet-{target}/p2pd
    let member = format!("kwaainet-{}/{}", target, binary_name);
    let status = std::process::Command::new("tar")
        .args([
            "-xJf",
            archive.to_str().unwrap(),
            "--strip-components=1",
            "-C",
            out_dir.to_str().unwrap(),
            &member,
        ])
        .status()
        .context("failed to run tar — is it installed?")?;
    if !status.success() {
        bail!("tar exited with status {}", status);
    }
    let out = out_dir.join(binary_name);
    if !out.exists() {
        bail!("tar ran but {} was not found in output", binary_name);
    }
    Ok(out)
}

/// Extract `p2pd.exe` from a `.zip` archive using PowerShell's `Expand-Archive`.
#[allow(unused_variables)]
fn extract_from_zip(
    archive: &std::path::Path,
    out_dir: &std::path::Path,
    binary_name: &str,
) -> Result<std::path::PathBuf> {
    #[cfg(windows)]
    {
        let status = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!(
                    "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                    archive.display(),
                    out_dir.display()
                ),
            ])
            .status()
            .context("failed to run PowerShell Expand-Archive")?;
        if !status.success() {
            bail!("PowerShell Expand-Archive failed with status {}", status);
        }
        let out = out_dir.join(binary_name);
        if !out.exists() {
            bail!(
                "Expand-Archive ran but {} was not found in output",
                binary_name
            );
        }
        return Ok(out);
    }
    #[cfg(not(windows))]
    bail!("zip extraction is only supported on Windows");
}

/// Construct the Rust target triple for the current machine at runtime.
fn build_target_triple() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => Some("aarch64-apple-darwin"),
        ("macos", "x86_64") => Some("x86_64-apple-darwin"),
        ("linux", "x86_64") => Some("x86_64-unknown-linux-gnu"),
        ("linux", "aarch64") => Some("aarch64-unknown-linux-gnu"),
        ("windows", "x86_64") => Some("x86_64-pc-windows-msvc"),
        _ => None,
    }
}

/// Search PATH for `name`, returning the full path if found.
fn find_in_path(name: &str) -> Option<std::path::PathBuf> {
    let paths = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&paths) {
        let candidate = dir.join(name);
        if candidate.exists() {
            return Some(candidate);
        }
    }
    None
}
