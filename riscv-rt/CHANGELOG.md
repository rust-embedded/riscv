# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

### Changed

- Limit rustc cfg flags to `riscvi`, `riscvm`, `riscvf`, and `riscvd`.
- Temporary use of `RISCV_RT_LLVM_ARCH_PATCH` environment variable to include the
  temporary patch required for avoid LLVM spurious errors.
- `riscv-rt` now use the `RISCV_RT_BASE_ISA` environment variable to configure the behavior
  of `riscv-rt-macros` depending on aspects of the base ISA (e.g., RV32I or RV32E).
- Use `riscv-target-parser` in build script to identify target-specific configurations.
- Add documentation to trap frame fields.
- Avoid using `t3`+ in startup assembly to ensure compatibility with RVE.
- `link.x.in`: remove references to `eh_frame`.
- Rename start/end section symbols to align with `cortex-m-rt`:
    - `_stext`: it remains, as linker files can modify it.
    - `__stext`: it coincides with `_stext`.
    - `__etext`: new symbol. It points to the end of the text section.
    - `__srodata`: new symbol. It points to the start of the read-only data section.
    - `__erodata`: new symbol. It points to the end of the read-only data section.
    - `__sdata`: substitutes `_sdata`. It points to the start of the on-flash data section.
    - `__edata`: substitutes `_edata`. It points to the end of the on-flash data section.
    - `__idata`: substitutes `_idata`. It points to the start of the on-RAM data section.
    - `__sbss`: substitutes `_sbss`. It points to the start of the BSS section.
    - `__ebss`: substitutes `_ebss`. It points to the end of the BSS section.
    - `__sheap`: substitutes `_sheap`. It points to the start of the heap section.
    - `__eheap`: substitutes `_eheap`. It points to the end of the heap section.
    - `__estack`: substitutes `_estack`. It points to the end of the stack section.
    - `__sstack`: substitutes `_sstack`. It points to the start of the stack section.
- `__edata` and `__ebss` are now defined outside of their respective sections.
  In this way, users can inject custom sections and benefit from the copying and
  zeroing routines, respectively.
- As `__sheap` is now private, `riscv-rt` now provides a `heap_start` function to
  allow users get the initial address of the heap when initializing an allocator.
- Update documentation.
- Removed `.init.rust` section, as it is no longer required.

## [v0.13.0] - 2024-10-19

### Added

- Add integration tests to check that macros work as expected.
- Add `no-exceptions` feature to opt-out the default implementation of `_dispatch_exception`
- Add `no-interrupts` feature to opt-out the default implementation of `_dispatch_core_interrupt`
- Add `pre_init_trap` to detect early errors during the boot process.
- Add `v-trap` feature to enable interrupt handling in vectored mode.
- Add `core_interrupt` proc macro to help defining core interrupt handlers.
  If `v-trap` feature is enabled, this macro also generates its corresponding trap.
- Add `external_interrupt` proc macro to help defining external interrupt handlers.
- Add `exception` proc macro to help defining exception handlers.
  If `v-trap` feature is enabled, this macro also generates its corresponding trap.
- Add `u-boot` feature, so that you can start your elf binary with u-boot and
work with passed arguments.

### Changed

- Use `cfg_attr` in `start_trap_rust` to allow compilation in non-riscv targets.
- Moved all the assembly code to `asm.rs`
- Use `weak` symbols for functions such as `_mp_hook` or `_start_trap`
- `abort` is now `weak`, so it is possible to link third-party libraries including this symbol.
- Made `cfg` variable selection more robust for custom targets
- `_start_trap_rust` now relies on `_dispatch_exception` and `_dispatch_core_interrupt`.
  This change allows more flexibility for targets with non-standard exceptions and interrupts.
- Upgrade rust-version to 1.61
- Update `syn` to version 2.0

### Removed

- `start_rust` is no longer needed, as it is now written in assembly
- `default_*` symbols are no longer needed, as we use `weak` symbols now.

## [v0.12.2] - 2024-02-15

### Added

- Implementation of `default_mp_hook` when `single-hart` feature is enabled.

## [v0.12.1] - 2024-01-24

### Added

- Patch in assembly code to avoid spurious errors from LLVM

## [v0.12.0] - 2024-01-14

### Added

- Add `links` field in `Cargo.toml`
- Add FPU initialization
- Static array for vectored-like handling of exceptions
- New GitHub workflow for checking invalid labels in PRs
- New GitHub workflow for checking modifications on CHANGELOG.md
- New GitHub workflow for checking clippy lints in PRs
- Optional cargo feature `single-hart` for single CPU targets

### Changed

- Removed _start_rust. Now, assembly directly jumps to main
- Removed U-mode interrupts to align with latest RISC-V specification
- Changed `Vector` union. Now, it uses `Option<fn>`, which is more idiomatic in Rust
- Removed riscv-target dependency for build
- Upgrade rust-version to 1.60
- Cargo workspace for riscv and riscv-rt
- Use inline assembly instead of pre-compiled blobs
- Removed bors in favor of GitHub Merge Queue
- `start_trap_rust` is now marked as `unsafe`
- Implement `r0` as inline assembly
- Use `${ARCH_WIDTH}` in `link.x.in` to adapt to different archs
- mhartid CSR is no longer read in single-hart mode, assumed zero
- Ensure stack pointer is 16-byte aligned before jumping to Rust entry point

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
