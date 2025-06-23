[![crates.io](https://img.shields.io/crates/d/riscv-semihosting.svg)](https://crates.io/crates/riscv-semihosting)
[![crates.io](https://img.shields.io/crates/v/riscv-semihosting.svg)](https://crates.io/crates/riscv-semihosting)

# `riscv-semihosting`

> Simple semihosting for RISC-V processors

This is a fork of the
[`cortex-m-semihosting`] crate with changes
to support the RISC-V Semihosting Specification as documented
[here](https://github.com/riscv/riscv-semihosting-spec/blob/main/riscv-semihosting-spec.adoc)

This crate can (almost) be used in exactly the same way as cortex-m-semihosting,
simply by changing calls to `cortex_m_semihosting::*` to `riscv_semihosting::*`.
Given this, the
[`cortex-m-semihosting documentation`](https://docs.rs/cortex-m-semihosting) is
generally sufficient for using this library.

A major difference between this library and `cortex-m-semihosting` is that there
are features to choose the privilege level at which the semihosting
calls are executed. The *machine-mode (M-mode)* feature will cause the macros in `export`
to execute the semihosting operation in an interrupt-free context, while
*user-mode (U-mode)* causes them to just execute the operation.
By default, M-mode is used. You can activate the U-mode via the `u-mode` feature.

# About the [`semihosting`] crate

`riscv-semihosting` provides a simple semihosting API that matches [`cortex-m-semihosting`].
This allows a simple port from Cortex-M applications to RISC-V applications.
However, the [`semihosting`] crate presents a more advanced interface that is compatible
for RISC-V as well as other architectures (e.g., ARM or MIPS).
While `riscv-semihosting` is a good starting point for developing semihosted applications,
**we recommend using the [`semihosting`] crate.**


# Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.85.0 and up. It **won't**
compile with older versions.

## License

Copyright 2018-2025 [RISC-V team][team]

Permission to use, copy, modify, and/or distribute this software for any purpose
with or without fee is hereby granted, provided that the above copyright notice
and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS
OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.

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
[`semihosting`]: https://crates.io/crates/semihosting
[`cortex-m-semihosting`]: https://docs.rs/cortex-m-semihosting
