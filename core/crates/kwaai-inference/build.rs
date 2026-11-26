//! `cfg(mlx)`: the `mlx` feature on Apple Silicon, the only target MLX builds for.
//! `cfg(mlx_embedded_metallib)`: mlx.metallib was found, so it is embedded.

use std::{env, path::Path};

fn main() {
    println!("cargo:rustc-check-cfg=cfg(mlx)");
    println!("cargo:rustc-check-cfg=cfg(mlx_embedded_metallib)");
    println!("cargo:rerun-if-env-changed=MLX_RS_METAL_PATH");

    let apple_silicon = env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos")
        && env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("aarch64");
    if env::var_os("CARGO_FEATURE_MLX").is_none() || !apple_silicon {
        return;
    }
    println!("cargo:rustc-cfg=mlx");

    // mlx-sys builds the kernels into MLX_RS_METAL_PATH (core/.cargo/config.toml).
    // Without it they land in a per-user directory, so MLX needs them beside the binary.
    let Some(dir) = env::var_os("MLX_RS_METAL_PATH") else {
        println!(
            "cargo:warning=MLX_RS_METAL_PATH unset; MLX will need mlx.metallib beside the binary"
        );
        return;
    };
    let path = Path::new(&dir).join("mlx.metallib");
    let path = path.to_str().expect("MLX_RS_METAL_PATH is UTF-8");
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("reading {path}: {e}"));
    println!("cargo:rerun-if-changed={path}");
    println!("cargo:rustc-cfg=mlx_embedded_metallib");
    println!("cargo:rustc-env=KWAAI_MLX_METALLIB={path}");
    println!(
        "cargo:rustc-env=KWAAI_MLX_METALLIB_ID={:016x}",
        fnv1a(&bytes)
    );
}

fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x100000001b3)
    })
}
