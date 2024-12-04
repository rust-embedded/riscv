# RISC-V crates

This repository contains various crates useful for writing Rust programs on RISC-V microcontrollers:

* [`riscv`]: CPU registers access and intrinsics
* [`riscv-pac`]: Common traits to be implemented by RISC-V PACs
* [`riscv-peripheral`]: Interfaces for standard RISC-V peripherals
* [`riscv-rt`]: Startup code and interrupt handling
* [`riscv-semihosting`]: Semihosting for RISC-V processors
* [`riscv-target-parser`]: Utility crate for parsing RISC-V targets in build scripts

This project is developed and maintained by the [RISC-V team][team].

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [RISC-V team][team], promises
to intervene to uphold that code of conduct.

[`riscv`]: https://crates.io/crates/riscv
[`riscv-pac`]: https://crates.io/crates/riscv-pac
[`riscv-peripheral`]: https://crates.io/crates/riscv-peripheral
[`riscv-rt`]: https://crates.io/crates/riscv-rt
[`riscv-semihosting`]: https://crates.io/crates/riscv-semihosting
[`riscv-target-parser`]: https://crates.io/crates/riscv-target-parser
[team]: https://github.com/rust-embedded/wg#the-risc-v-team
[CoC]: CODE_OF_CONDUCT.md
