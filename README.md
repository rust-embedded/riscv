[![crates.io](https://img.shields.io/crates/v/cortex-m-semihosting.svg)](https://crates.io/crates/cortex-m-semihosting)
[![crates.io](https://img.shields.io/crates/d/cortex-m-semihosting.svg)](https://crates.io/crates/cortex-m-semihosting)

# `riscv-semihosting`

> Semihosting for RISC-V processors

This is a fork of the
[cortex-m-semihosting](https://docs.rs/cortex-m-semihosting) crate with minimal
changes to support the RISC-V Semihosting Specification as documented
[here](https://github.com/riscv/riscv-semihosting-spec/blob/main/riscv-semihosting-spec.adoc)

This crate can be used in exactly the same way as cortex-m-semihosting, simply
by changing calls to `cortex_m_semihosting::*` to `riscv_semihosting::*`.

The rest of this document is as-is from upstream, and obviously any
ARM-specific sections should be ignored.

This project is developed and maintained by the [Cortex-M team][team].

## [Documentation](https://docs.rs/cortex-m-semihosting)

# Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.59.0 and up. It **won't**
compile with older versions.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Cortex-M team][team], promises
to intervene to uphold that code of conduct.

[CoC]: ../CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-cortex-m-team
