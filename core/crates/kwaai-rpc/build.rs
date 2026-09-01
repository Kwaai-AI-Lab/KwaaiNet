//! Compile kwaai.proto into both server and client stubs via tonic-build.
//!
//! Mirrors the protoc-bootstrap approach used by kwaai-p2p-daemon's build.rs
//! so a developer without a system protoc still gets a working build: if
//! protoc isn't on PATH, we download a pinned release into OUT_DIR/protoc and
//! point tonic-build at that binary via $PROTOC.

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// protoc 28.3 release archives, sha256. Pinned per platform for the same
/// reason `core/patches/*.sh` pin theirs: an unverified download that arrives
/// short is indistinguishable from one that arrived whole until something
/// downstream fails for an unrelated-looking reason.
const PROTOC_VERSION: &str = "28.3";
const PROTOC_SHA256: &[(&str, &str)] = &[
    (
        "win64",
        "ce64f49bdeddef49ce4bd313a8f59bcf92fcf67b5831efbf66170386d2e66948",
    ),
    (
        "osx-aarch_64",
        "92ceefda6a7293ec014e6ecac82d64719357145cb6fc2865badadeb5e62c0431",
    ),
    (
        "osx-x86_64",
        "97fe5d442090b4dbc23cd1384fb9b444fa1dc6e67d15bb5e1fe4de0da7638b20",
    ),
    (
        "linux-x86_64",
        "0ad949f04a6a174da83cdcbdb36dee0a4925272a5b6d83f79a6bf9852076d53f",
    ),
    (
        "linux-aarch_64",
        "1de522032a8b194002fe35cab86d747848238b5e4de4f99648372079f5b46f9a",
    ),
];

/// Hex sha256 of a file, computed here so the build script needs no
/// dependency and no shelling out to a hashing tool that may not exist.
fn sha256_hex(path: &Path) -> std::io::Result<String> {
    // Minimal FIPS 180-4 SHA-256 over a streamed file.
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let data = std::fs::read(path)?;
    let bit_len = (data.len() as u64).wrapping_mul(8);
    let mut msg = data;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, word) in chunk.chunks_exact(4).enumerate() {
            w[i] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let t1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (slot, v) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *slot = slot.wrapping_add(v);
        }
    }
    Ok(h.iter().map(|w| format!("{w:08x}")).collect())
}

/// A usable protoc already on this machine, if there is one.
///
/// `$PROTOC` first: it is the variable prost-build itself names in its
/// "could not find protoc" error, so an operator who has protoc somewhere
/// other than PATH expects it to be honoured — and this function ends by
/// setting that very variable.
///
/// Both probes check the exit *status*. `Command::output()` returning `Ok`
/// only means the process spawned; a protoc that starts and fails counts as
/// present under that test, and the build then dies much later inside
/// tonic-build with the early-out already taken.
fn existing_protoc() -> Option<PathBuf> {
    if let Ok(configured) = env::var("PROTOC") {
        if !configured.is_empty() {
            let path = PathBuf::from(&configured);
            match Command::new(&path).arg("--version").output() {
                Ok(out) if out.status.success() => return Some(path),
                _ => panic!(
                    "kwaai-rpc build: PROTOC is set to {configured:?}, but running it with \
                     --version failed. Fix or unset PROTOC; leaving it set to something \
                     unusable would otherwise be silently replaced by a download."
                ),
            }
        }
    }
    match Command::new("protoc").arg("--version").output() {
        Ok(out) if out.status.success() => Some(PathBuf::from("protoc")),
        _ => None,
    }
}

fn ensure_protoc(out_dir: &Path) {
    if let Some(found) = existing_protoc() {
        env::set_var("PROTOC", found);
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
             Install protoc manually (or set PROTOC) and re-run cargo build."
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

        let url = format!(
            "https://github.com/protocolbuffers/protobuf/releases/download/v{PROTOC_VERSION}/protoc-{PROTOC_VERSION}-{platform}.{archive_ext}"
        );
        let archive_path = protoc_dir.join(format!("protoc.{archive_ext}"));

        let download_ok = if cfg!(windows) {
            // `Invoke-WebRequest` throws on a non-2xx, which `-ErrorAction
            // Stop` turns into a non-zero exit; without it the cmdlet can
            // report failure and PowerShell still exits 0.
            let cmd = format!(
                "$ProgressPreference='SilentlyContinue'; Invoke-WebRequest -Uri '{}' -OutFile '{}' -ErrorAction Stop",
                url,
                archive_path.display()
            );
            Command::new("powershell")
                .args(["-NoProfile", "-Command", &cmd])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        } else {
            // `--fail` is the whole point: without it curl writes a 404 body
            // to the output file and exits 0, so the archive is "downloaded"
            // and the failure surfaces later as a corrupt zip.
            Command::new("curl")
                .args(["--fail", "--location", "--silent", "--show-error", "-o"])
                .arg(&archive_path)
                .arg(&url)
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        };
        if !download_ok {
            let _ = std::fs::remove_file(&archive_path);
            panic!("kwaai-rpc build: failed to download protoc from {url}");
        }

        // Exit status says the transfer ended, not that it was complete. A
        // truncated archive keeps a valid header and only fails at extraction,
        // as an "End of Central Directory record could not be found" that
        // names neither the download nor the file.
        let expected = PROTOC_SHA256
            .iter()
            .find(|(p, _)| *p == platform)
            .map(|(_, sha)| *sha)
            .unwrap_or_else(|| {
                panic!("kwaai-rpc build: no pinned sha256 for protoc platform {platform}")
            });
        let actual = sha256_hex(&archive_path)
            .unwrap_or_else(|e| panic!("kwaai-rpc build: cannot read the protoc archive: {e}"));
        if actual != expected {
            let size = std::fs::metadata(&archive_path)
                .map(|m| m.len())
                .unwrap_or(0);
            // Removed so a re-run re-downloads rather than tripping over it.
            let _ = std::fs::remove_file(&archive_path);
            panic!(
                "kwaai-rpc build: protoc archive from {url} does not match its pinned \
                 sha256.\n  expected {expected}\n  actual   {actual}  ({size} bytes)\n\
                 A short file means the download was interrupted — re-run the build. \
                 If it is reproducible, the pinned hash may need updating for a \
                 re-published release."
            );
        }

        let extract_ok = if cfg!(windows) {
            let cmd = format!(
                "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                archive_path.display(),
                protoc_dir.display()
            );
            Command::new("powershell")
                .args(["-NoProfile", "-Command", &cmd])
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

        // End-to-end check: the archive hashed correctly and unpacked, but
        // only running the binary proves this build has a working protoc.
        match Command::new(&protoc_bin).arg("--version").output() {
            Ok(out) if out.status.success() => {}
            other => panic!(
                "kwaai-rpc build: extracted protoc at {} does not run ({other:?}). \
                 Install protoc manually, or set PROTOC to a working one.",
                protoc_bin.display()
            ),
        }
    }

    env::set_var("PROTOC", &protoc_bin);
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=proto/kwaai.proto");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    ensure_protoc(&out_dir);

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let proto_dir = manifest_dir.join("proto");
    let proto = proto_dir.join("kwaai.proto");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&[proto], &[proto_dir])
        .expect("compile kwaai.proto");
}
