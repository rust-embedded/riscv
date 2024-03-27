# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Added

- Add `Mcause::from(usize)` for use in unit tests
- Add `Mstatus.bits()`

### Fixed

- Fixed `sip::set_ssoft` and `sip::clear_ssoft` using wrong address

## [v0.11.1] - 2024-02-15

### Changed

- Made `asm::wfi`, `fence`, `fence_i` and `sfence` safe (ie, removed `unsafe` from their definitions)

## [v0.11.0] - 2024-01-14

### Added

- Add `asm::ecall()`, a wrapper for implementing an `ecall` instruction
- Add `nested` function for nested ISRs in `interrupt::machine` and `interrupt::supervisor`
- `s-mode` feature for reexporting `interrupt::machine` or `interrupt::supervisor` to `interrupt`
- Support for supervisor-level interrupts in `interrupt::supervisor`
- Add CI workflow to check that CHANGELOG.md file has been modified in PRs
- Add `read_csr_as_rv32`, `set_rv32`, and `clear_rv32` macros
- Add `mstatus::uxl` and `mstatus::sxl`
- Add `mstatus::ube`, `mstatus::sbe`, and `mstatus::mbe` endianness bit fields
- Add `mstatush` registers (RISCV-32 only)
- Add `asm::fence()`, a wrapper for implementing a `fence` instruction
- Add `asm::fence_i()`, a wrapper for implementing a `fence.i` instruction
- Add `TryFrom` implementation for `mcause::{Interrupt, Exception}` and `scause::{Interrupt, Exception}`

### Changed

- Cargo workspace for riscv and riscv-rt
- Update `embedded-hal` dependency to v1.0.0 (bumps MSRV to 1.60)
- `misa::MXL` renamed to `misa::XLEN`
- Removed `bit_field` dependency
- CI actions updated. They now use `checkout@v3` and `dtolnay/rust-toolchain`.
- `mcause::{Interrupt, Exception}` and `scause::{Interrupt, Exception}` now implement `From` trait for `usize`
- Set safety of `asm::nop` and `asm::delay` functions to safe.

### Fixed

- Fix `scause::Exception` missing `LoadMisaligned`
- Fix `scause::Exception` missing `SupervisorEnvCall`
- Removed user-level interrupts from `mcause::Interrupt` and `scause::Interrupt`
- Removed user-level interrupts from `mstatus`
- Removed machine environment call delegation from `medeleg`
- Removed user-level interrupts from machine and supervisor mode interrupt-related registers.

### Removed

- User mode registers removed, as they are no longer supported in RISC-V
- FCSR register operations removed to avoid UB (#148)

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
