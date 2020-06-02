# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

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


[Unreleased]: https://github.com/rust-embedded/riscv-rt/compare/v0.7.1...HEAD
[v0.7.1]: https://github.com/rust-embedded/riscv/compare/v0.7.0...v0.7.1
[v0.7.0]: https://github.com/rust-embedded/riscv/compare/v0.6.1...v0.7.0
