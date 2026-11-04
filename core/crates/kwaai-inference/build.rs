//! `cfg(mlx)`: the `mlx` feature on Apple Silicon, the only target MLX builds for.
//! `cfg(mlx_embedded_metallib)`: mlx-sys reported its kernels, so they are embedded.

use std::env;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(mlx)");
    println!("cargo:rustc-check-cfg=cfg(mlx_embedded_metallib)");
    println!("cargo:rerun-if-env-changed=DEP_MLX_METALLIB");

    let apple_silicon = env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos")
        && env::var("CARGO_CFG_TARGET_ARCH").as_deref() == Ok("aarch64");
    if env::var_os("CARGO_FEATURE_MLX").is_none() || !apple_silicon {
        return;
    }
    println!("cargo:rustc-cfg=mlx");

    // Only the patched mlx-sys (core/patches) reports this; a stock one falls
    // back to MLX's own lookup beside the binary.
    let Ok(path) = env::var("DEP_MLX_METALLIB") else {
        println!(
            "cargo:warning=mlx-sys did not report mlx.metallib; MLX will need it beside the binary"
        );
        return;
    };
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("reading {path}: {e}"));
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
