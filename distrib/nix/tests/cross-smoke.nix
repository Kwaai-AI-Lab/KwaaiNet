# QEMU user-mode smoke test for cross-compiled binaries.
#
# Verifies that cross-compiled kwaainet binaries can execute --help and
# --version, either natively (same CPU) or via QEMU user-mode emulation.
{
  pkgs,
  kwaainet,
  arch, # e.g., "aarch64" or "x86_64"
  isStatic ? false,
}:

let
  lib = pkgs.lib;

  qemuBin =
    {
      "aarch64" = "qemu-aarch64";
      "riscv64" = "qemu-riscv64";
      "x86_64" = null; # native, no QEMU needed
    }
    .${arch};

  needsQemu = qemuBin != null;

  # Static binaries don't need LD_PREFIX; dynamic ones need the cross libc.
  runner =
    if !needsQemu then
      "${kwaainet}/bin/kwaainet"
    else if isStatic then
      "${pkgs.qemu-user}/bin/${qemuBin} ${kwaainet}/bin/kwaainet"
    else
      "${pkgs.qemu-user}/bin/${qemuBin} -L ${pkgsCross.stdenv.cc.libc} ${kwaainet}/bin/kwaainet";

  # For dynamic aarch64 binaries, we need the cross libc for QEMU.
  pkgsCross =
    if needsQemu && !isStatic then
      import pkgs.path {
        localSystem = "x86_64-linux";
        crossSystem = {
          config = "${arch}-unknown-linux-gnu";
        };
      }
    else
      null;

  staticSuffix = if isStatic then "-static" else "";
  testName = "kwaainet-cross-smoke-${arch}${staticSuffix}";
in
pkgs.runCommand testName
  {
    nativeBuildInputs = lib.optional needsQemu pkgs.qemu-user;
  }
  ''
    ${runner} --help > /dev/null
    echo "PASS: --help (${arch}${staticSuffix})"

    ${runner} --version
    echo "PASS: --version (${arch}${staticSuffix})"

    echo ""
    echo "All cross smoke tests passed for ${arch}${staticSuffix}."
    touch $out
  ''
