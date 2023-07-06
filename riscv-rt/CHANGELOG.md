# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

- Removed bors in favor of GitHub Merge Queue

## [v0.11.0] - 2023-01-18

### Changed

- Update `riscv` to version 0.10.1 fixing a critical section bug

## [v0.10.0] - 2022-11-04

### Added

- Optional cargo feature `s-mode` for supervisor mode, including conditional compilation for supervisor/machine mode instructions.

### Changed

- Remove superfluous parentheses from link.x, which caused linker errors with nightly.
- Changed `mp_hook` signature, hartid as passed as usize parameter by the caller (required for `s-mode` feature).
- Update `riscv` to version 0.9

## [v0.9.0] - 2022-07-01

### Added

- Pass `a0..a2` register values to the `#[entry]` function.

### Changed

- Update `riscv` to version 0.8
- Update `riscv-rt-macros` to 0.2.0
- Update Minimum Supported Rust Version to 1.59
- The main symbol is no longer randomly generated in the `#[entry]` macro, instead it uses `__risc_v_rt__main`.

### Removed

- Remove `inline-asm` feature which is now always enabled

## [v0.8.1] - 2022-01-25

### Added

- Enable float support for targets with extension sets F and D
- Add ability to override trap handling mechanism

### Changed

- Update `riscv` to version 0.7
- Update `quote` to version 1.0
- Update `proc-macro2` to version 1.0
- Update `rand` to version to version 0.7.3

## [v0.8.0] - 2020-07-18

### Changed

- Update `riscv` to version 0.6
- Update Minimum Supported Rust Version to 1.42.0

## [v0.7.2] - 2020-07-16

### Changed

- Preserve `.eh_frame` and `.eh_frame_hdr` sections
- Place `.srodata` and `.srodata.*` sections in `.rodata`

## [v0.7.1] - 2020-06-02

### Added

- Add support to initialize custom interrupt controllers.

### Changed

- Exception handler may return now

## [v0.7.0] - 2020-03-10

### Added

- Assure address of PC at startup
- Implement interrupt and exception handling
- Add support for the `riscv32i-unknown-none-elf` target
- Added Changelog

### Fixed

- Fix linker script compatibility with GNU linker

### Changed

- Move `abort` out of the `.init` section
- Update `r0` to v1.0.0
- Set MSRV to 1.38


[Unreleased]: https://github.com/rust-embedded/riscv-rt/compare/v0.11.0..HEAD
[v0.10.1]: https://github.com/rust-embedded/riscv-rt/compare/v0.10.0...v0.11.0
[v0.10.0]: https://github.com/rust-embedded/riscv-rt/compare/v0.9.1...v0.10.0
[v0.9.0]: https://github.com/rust-embedded/riscv-rt/compare/v0.8.1...v0.9.0
[v0.8.1]: https://github.com/rust-embedded/riscv-rt/compare/v0.8.0...v0.8.1
[v0.8.0]: https://github.com/rust-embedded/riscv-rt/compare/v0.7.2...v0.8.0
[v0.7.2]: https://github.com/rust-embedded/riscv-rt/compare/v0.7.1...v0.7.2
[v0.7.1]: https://github.com/rust-embedded/riscv-rt/compare/v0.7.0...v0.7.1
[v0.7.0]: https://github.com/rust-embedded/riscv-rt/compare/v0.6.1...v0.7.0
