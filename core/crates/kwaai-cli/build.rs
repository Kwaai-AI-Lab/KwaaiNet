//! `cfg(mlx)`: the `mlx` feature on Apple Silicon. Elsewhere the feature is inert,
//! so one feature list (the release's) can serve every target.

fn main() {
    println!("cargo:rustc-check-cfg=cfg(mlx)");
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    if std::env::var_os("CARGO_FEATURE_MLX").is_some() && os == "macos" && arch == "aarch64" {
        println!("cargo:rustc-cfg=mlx");
    }
}
