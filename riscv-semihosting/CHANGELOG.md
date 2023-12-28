# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

- Bring in API changes from
  [cortex-m-semihosting](https://github.com/rust-embedded/cortex-m/tree/master/cortex-m-semihosting),
  including:
    - Addition of the `hprint`, `hprintln`, `heprint`, `heprintln`, and `dbg`
      macros.
        - `hprint` and `heprintln` print to host stdout without and with a
          newline, respectively.
        - `heprint` and `heprintln` do the same, except to host stderr.
        - `dbg` works exactly like
          [`std::dbg`](https://doc.rust-lang.org/std/macro.dbg.html).
    - `HStdout` and `HStderr` have been combined into `HostStream`.
    - `inline-asm` feature removed, switched to stabilized inline asm and MSRV
      bumped to 1.59.0
- Clean up documentation, removing unnecessary references to
  cortex-m-semihosting and improving clarity.
- Added GitHub Actions CI
- Add features to select the privilege level the semihosting operations will be
  started from

## [v0.0.1] - 2018-02-27

- Initial release

[Unreleased]: https://github.com/riscv-rust/riscv-semihosting/compare/cb1afe4002d576b87bfd4c199f42a43815984ce4..HEAD
[v0.0.1]: https://github.com/riscv-rust/riscv-semihosting/tree/cb1afe4002d576b87bfd4c199f42a43815984ce4
