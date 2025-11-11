# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- New `rt` and `rt-v-trap` features to opt-in `riscv-rt`-related code in `riscv::pac_enum` macro.

### Changed

- Use fully qualified paths in generated code (i.e., `::riscv` instead of `riscv`)
- Moved from `riscv/macros/` to `riscv-macros/`
- Now, `riscv::pac_enum` macro only includes trap-related code if `rt` or `rt-v-trap` features are enabled.

## [v0.3.0] - 2025-09-08

This crate was placed inside `riscv/`. Check `riscv/CHANGELOG.md` for details
