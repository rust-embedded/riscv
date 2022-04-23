# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

- Update `riscv` to version 0.8
- Update Minimum Supported Rust Version to 1.59

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


[Unreleased]: https://github.com/rust-embedded/riscv-rt/compare/v0.8.1..HEAD
[v0.8.1]: https://github.com/rust-embedded/riscv/compare/v0.8.0...v0.8.1
[v0.8.0]: https://github.com/rust-embedded/riscv/compare/v0.7.2...v0.8.0
[v0.7.2]: https://github.com/rust-embedded/riscv/compare/v0.7.1...v0.7.2
[v0.7.1]: https://github.com/rust-embedded/riscv/compare/v0.7.0...v0.7.1
[v0.7.0]: https://github.com/rust-embedded/riscv/compare/v0.6.1...v0.7.0
