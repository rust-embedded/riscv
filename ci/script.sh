set -euxo pipefail

main() {
    local cargo=
    if [ $TARGET = thumbv7m-none-eabi ]; then
        cargo=xargo
    else
        cargo=cargo
    fi

    $cargo check --target $TARGET
}

main
