# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- Added `riscv32i-unknown-none-elf` target support
- Added user trap setup and handling registers
- Added write methods for the `mip` and `satp` registers
- Added `mideleg` register
- Added Changelog

### Changed

- Fixed MSRV by restricting the upper bound of `bare-metal` version

[Unreleased]: https://github.com/rust-embedded/riscv/compare/v0.5.4...HEAD
