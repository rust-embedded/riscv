# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased

## v0.4.1 - 2026-05-29

### Added

- New `setup_interrupts` macro for custom interrupt setup routines.
- New `exception`, `core_interrupt`, and `external_interrupt` macros for trap handlers.
- New `entry` macro for the Rust entry point (for `riscv-rt`).
- New `rvrt-u-boot` feature to adapt `entry` macro for U-Boot.
- New `post_init` macro for Rust routines that must be executed before main.
- New `rvrt_llvm_arch_patch` and `rvrt_default_start_trap` for generating assembly code
  required by the `riscv-rt` crate.
- New `riscv-rt` feature to opt-in `riscv-rt`-related macros.
- New `rvrt-pre-default-start-trap` feature to opt-in assembly injection at the
  beginning of `_default_start_trap`.
- New `s-mode` feature to adapt macros to S-Mode execution.

## v0.4.0 - 2025-12-19

### Added

- New `rt` and `rt-v-trap` features to opt-in `riscv-rt`-related code in `riscv::pac_enum` macro.

### Changed

- Isolate code that depend on the `riscv` crate in a new `riscv` module.
- Fix `cargo doc` errors.
- Use fully qualified paths in generated code (i.e., `::riscv` instead of `riscv`)
- Moved from `riscv/macros/` to `riscv-macros/`
- Now, `riscv::pac_enum` macro only includes trap-related code if `rt` or `rt-v-trap` features are enabled.

## [v0.3.0] - 2025-09-08

This crate was placed inside `riscv/`. Check `riscv/CHANGELOG.md` for details
