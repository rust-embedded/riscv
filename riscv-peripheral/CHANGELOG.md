# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- use `riscv-pac` result types for trait implementations

### Fixed

- `clippy` fixes

### Changed

- `PLIC` now expects interrupt enums to implement the `riscv_pac::ExternalInterruptNumber` trait.

## [v0.1.0] - 2024-02-15

### Added

- Add `ACLINT`, `CLINT`, and `PLIC` structs
