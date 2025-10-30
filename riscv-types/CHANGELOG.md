# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

- Renamed crate to `riscv-types` as per [#351](https://github.com/rust-embedded/riscv/issues/351)

## riscv-pac [v0.2.0] - 2024-10-19

### Added

- Add `result` module for `Error` and `Result` types
- Add `ExceptionNumber` trait.
- Classify interrupt numbers in `CoreInterruptNumber` and `ExternalInterruptNumber`.
- Added simple tests to illustrate how to implement all the provided traits.

### Changed 

- All traits now work with `usize` data type.

## riscv-pac [v0.1.1] - 2024-02-15

- Fix crates.io badge links

## riscv-pac [v0.1.0] - 2024-01-14

### Added

- Add `InterruptNumber`, `PriorityNumber`, and `HartIdNumber` traits.

### Changed

- Update `README.md`
