# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

- Use `cfg(any(target_arch = "riscv32", target_arch = "riscv64"))` instead of `cfg(riscv)`.

### Removed

- Removed custom build script, as `cfg(riscv)` is no longer necessary.

## [v0.2.0] - 2025-06-10

### Changed

- Update riscv to 0.14.0
- Bump MSRV to 1.67

## [v0.1.3] - 2025-02-18

### Changed

- Update riscv to 0.13.0

## [v0.1.2] - 2024-10-20

### Changed

- Update critical-section to 1.2.0

## [v0.1.1] - 2024-10-19

### Changed

- Apply clippy changes
- Bump riscv dependency version
- Made `cfg` variable selection more robust for custom targets
- Fixed debug::exit() on riscv64 QEMU simulation
- Fixed an ambiguous link in the generated crate documentation.

## [v0.1.0] - 2023-01-18

- Add recommendation for `semihosting` in README.md.
- Bug fixes
- Moved to the `riscv` Cargo workspace
- Bring in API changes from
  [cortex-m-semihosting](https://github.com/rust-embedded/cortex-m/tree/master/cortex-m-semihosting),
  including:
    - Addition of the `hprint`, `hprintln`, `heprint`, `heprintln`, and `dbg`
      macros.
        - `hprint` and `heprintln` print to host stdout without and with a
          newline, respectively.
        - `heprint` and `heprintln` do the same, except to host stderr.
        - `dbg` works exactly like
          [`std::dbg`](https://doc.rust-lang.org/std/macro.dbg.html).
    - `HStdout` and `HStderr` have been combined into `HostStream`.
    - `inline-asm` feature removed, switched to stabilized inline asm and MSRV
      bumped to 1.59.0
- Clean up documentation, removing unnecessary references to
  cortex-m-semihosting and improving clarity.
- Added GitHub Actions CI
- Add features to select the privilege level the semihosting operations will be
  started from

## [v0.0.1] - 2018-02-27

- Initial release

[Unreleased]: https://github.com/riscv-rust/riscv-semihosting/compare/cb1afe4002d576b87bfd4c199f42a43815984ce4..HEAD
[v0.0.1]: https://github.com/riscv-rust/riscv-semihosting/tree/cb1afe4002d576b87bfd4c199f42a43815984ce4
