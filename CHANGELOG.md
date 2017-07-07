# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [v0.2.0] - 2017-07-07

### Added

- `exit` and `report_exception` syscalls

- `HStdout` and `HStderr` structs that represent handles to the host stdout and
  stderr stream respectively.

### Changed

- [breaking-change] The `io` module has been renamed to `hio` to reflect that
  this is I/O *on the host*.

### Removed

- [breaking-change] the family of `write` functions in the `io` module. Instead
  use `HStdout` / `HStderr` and its `write_all` method and `fmt::Write`
  implementation.

- [breaking-change] the `hprint!` family of macros. Instead use `HStdout` and
  the standard `write!` macro.

## [v0.1.3] - 2017-02-27

### Added

- A family of `ewrite` functions and `ehprint!` macros to write to the host's
  stderr.

### Fixed

- `write_all` logic when a single write doesn't write all the buffer bytes

## [v0.1.2] - 2017-02-15

### Fixed

- the `hprintln!` macro when called without arguments.

## [v0.1.1] - 2017-01-22

### Added

- Expose a family of `write` functions to write to the host's stdout without
  going through the `hprint!` macros.

## v0.1.0 - 2017-01-22

- Initial release

[Unreleased]: https://github.com/japaric/cortex-m-semihosting/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/japaric/cortex-m-semihosting/compare/v0.1.3...v0.2.0
[v0.1.3]: https://github.com/japaric/cortex-m-semihosting/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/japaric/cortex-m-semihosting/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/japaric/cortex-m-semihosting/compare/v0.1.0...v0.1.1
