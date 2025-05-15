# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

- Rework of CLINT peripheral to use methods instead of associated functions.
  This change follows the `svd2rust` pattern, making the ecosystem more consistent.
- Simplify `clint!` macro using the `Deref` trait.

### Removed

- Removed support for `embedded-hal-async`, as it was not flexible enough to be
  used in different targets (single HART, multi HART...). Instead, each chip must
  have its own `chip-hal-async` crate that properly adapts to its specific needs.

### Fixed

- `clippy` fixes

## [v0.2.1] - 2025-02-18

### Changed

- Update `riscv` dependency to 0.13.0

## [v0.2.0] - 2024-10-19

### Added

- use `riscv-pac` result types for trait implementations

### Changed

- Adapt to new version of `riscv-pac` traits.
- `PLIC` now expects interrupt enums to implement the `riscv_pac::ExternalInterruptNumber` trait.

### Fixed

- `clippy` fixes

## [v0.1.0] - 2024-02-15

### Added

- Add `ACLINT`, `CLINT`, and `PLIC` structs
