#!/bin/bash

set -euxo pipefail

crate=riscv-rt

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

for ext in i ic im imc
do
    riscv64-unknown-elf-gcc -ggdb3 -fdebug-prefix-map=$(pwd)=/riscv-rt -c -mabi=ilp32 -march=rv32${ext} asm.S -o bin/$crate.o
    riscv64-unknown-elf-ar crs bin/riscv32${ext}-unknown-none-elf.a bin/$crate.o

    riscv64-unknown-elf-gcc -ggdb3 -fdebug-prefix-map=$(pwd)=/riscv-rt -c -mabi=lp64 -march=rv64${ext} asm.S -o bin/$crate.o
    riscv64-unknown-elf-ar crs bin/riscv64${ext}-unknown-none-elf.a bin/$crate.o
done

rm bin/$crate.o
