set -euxo pipefail

main() {
    cargo check --target $TARGET

    if [ $TRAVIS_RUST_VERSION = nightly ]; then
        cargo check --target $TARGET --features 'const-fn inline-asm'
    fi

    if [ $TARGET = x86_64-unknown-linux-gnu ]; then
        ./check-blobs.sh
    fi
}

main
