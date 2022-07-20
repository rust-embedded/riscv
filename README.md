# `riscv-semihosting`

> Semihosting for RISC-V processors

This is a fork of the
[cortex-m-semihosting](https://docs.rs/cortex-m-semihosting) crate with changes
to support the RISC-V Semihosting Specification as documented
[here](https://github.com/riscv/riscv-semihosting-spec/blob/main/riscv-semihosting-spec.adoc)

This crate can be used in exactly the same way as cortex-m-semihosting, simply
by changing calls to `cortex_m_semihosting::*` to `riscv_semihosting::*`. Given
this, the
[cortex-m-semihosting documentation](https://docs.rs/cortex-m-semihosting) is
generally sufficient for using this library.

# Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.59.0 and up. It **won't**
compile with older versions.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../LICENSE-MIT) or
  http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [RISC-V team][team], promises
to intervene to uphold that code of conduct.

[CoC]: ../CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-risc-v-team
