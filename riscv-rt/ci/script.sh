#!/usr/bin/env bash

set -euxo pipefail

if [ -n "${TARGET:-}" ]; then
    cargo check --target $TARGET

    if [[ $TARGET == riscv* ]]; then
        cargo check --target $TARGET --examples
    fi

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo check --target $TARGET --features inline-asm
    fi
fi

if [ -n "${CHECK_BLOBS:-}" ]; then
    PATH="$PATH:$PWD/gcc/bin"
    ./check-blobs.sh
fi

if [ -n "${RUSTFMT:-}" ]; then
    cargo fmt -- --check
fi
