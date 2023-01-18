# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.10.1] - 2023-01-18

### Fixed

- Fix implementation for `SingleHartCriticalSection`

## [v0.10.0] - 2022-11-09

### Added

- `critical-section-single-hart` feature which provides an implementation for the `critical_section` crate for single-hart systems, based on disabling all interrupts.

## [v0.9.0] - 2022-10-06

### Fixed

- Fix `asm::delay()` to ensure count register is always reloaded
- Fix reading marchid and mimpid (#107)

### Removed
- `set_msoft`, `clear_msoft`, `set_mtimer` and `clear_mtimer` removed as part of fixing issue #62

## [v0.8.0] - 2022-04-20

### Added

- Add `#[cfg(riscv32)]` to `pmpcfg1` and `pmpcfg3` modules
- Add enums `Range`, `Permission` for PMP configuration
- Add `set_pmp()` and `clear_pmp()` functions to pmpcfg(x) modules
- Add struct `Pmpcsr` and is returned from `pmpcfgx::read()`
- Add `singleton!` macro
- Add delay structure and methods using embedded-hal traits and `mcycle` register
- Add `asm::delay()` function for assembly-based busy-loops
- Add `asm::nop()`, a wrapper for implementing a `nop` instruction
- Add missing `#[inline]` attribute to register reads, type conversations and `interrupt::free`

### Changed

- Use new `asm!` instead of `llvm_asm!`
- Change `pmpcfgx::read()` macro to `read_csr_as!()` from `read_csr_as_usize!()`
- Inline assembly is now always used
- Update Minimum Supported Rust Version to 1.59

### Fixed

- Fix `sfence.vma` operand order

### Removed

- Remove `inline-asm` feature which is now always enabled

## [v0.7.0] - 2021-07-29

### Added

- Add `medeleg` register
- Add `cycle[h]`, `instret[h]` and `mcounteren`
- Add additional binaries for floating-point ABIs
- Add support for `mxr`
- Add support for `mprv`

### Changed

- Fix `scause::set`
- Various formatting and comment fixes
- Update `bare-metal` to `v1.0.0` removing `Nr` trait
- Build targets on `docs.rs` are now RISC-V targets other than default ones

## [v0.6.0] - 2020-06-20

### Changed

- `Mtvec::trap_mode()`, `Stvec::trap_mode()` and `Utvec::trap_mode()` functions now return `Option<TrapMode>` (breaking change)
- Updated Minimum Supported Rust Version to 1.42.0
- Use `llvm_asm!` instead of `asm!`

### Removed

- vexriscv-specific registers were moved to the `vexriscv` crate

## [v0.5.6] - 2020-03-14

### Added

- Added vexriscv-specific registers

## [v0.5.5] - 2020-02-28

### Added

- Added `riscv32i-unknown-none-elf` target support
- Added user trap setup and handling registers
- Added write methods for the `mip` and `satp` registers
- Added `mideleg` register
- Added Changelog

### Changed

- Fixed MSRV by restricting the upper bound of `bare-metal` version

[Unreleased]: https://github.com/rust-embedded/riscv/compare/v0.10.1...HEAD
[v0.10.1]: https://github.com/rust-embedded/riscv/compare/v0.10.0...v0.10.1
[v0.10.0]: https://github.com/rust-embedded/riscv/compare/v0.9.0...v0.10.0
[v0.9.0]: https://github.com/rust-embedded/riscv/compare/v0.8.0...v0.9.0
[v0.8.0]: https://github.com/rust-embedded/riscv/compare/v0.7.0...v0.8.0
[v0.7.0]: https://github.com/rust-embedded/riscv/compare/v0.6.0...v0.7.0
[v0.6.0]: https://github.com/rust-embedded/riscv/compare/v0.5.6...v0.6.0
[v0.5.6]: https://github.com/rust-embedded/riscv/compare/v0.5.5...v0.5.6
[v0.5.5]: https://github.com/rust-embedded/riscv/compare/v0.5.4...v0.5.5
