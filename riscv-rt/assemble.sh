#!/bin/bash

set -euxo pipefail

crate=riscv-rt

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

riscv64-unknown-elf-gcc -c -mabi=ilp32 -march=rv32imac asm.S -o bin/$crate.o
ar crs bin/riscv32imac-unknown-none-elf.a bin/$crate.o
cp bin/riscv32imac-unknown-none-elf.a bin/riscv32imc-unknown-none-elf.a

rm bin/$crate.o
