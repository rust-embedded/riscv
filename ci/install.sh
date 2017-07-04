set -euxo pipefail

main() {
    if [ $TARGET = thumbv7m-none-eabi ]; then
        cargo install --list | grep xargo || \
            cargo install xargo
        rustup component list | grep 'rust-src.*installed' || \
            rustup component add rust-src
    fi
}

main
