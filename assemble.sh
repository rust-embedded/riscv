#!/bin/bash

set -euxo pipefail

crate=riscv

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a

riscv64-unknown-elf-gcc -c -mabi=ilp32 -march=rv32i asm.S -o bin/$crate.o
ar crs bin/riscv32i-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=ilp32 -march=rv32ic asm.S -o bin/$crate.o
ar crs bin/riscv32ic-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64 -march=rv64i asm.S -o bin/$crate.o
ar crs bin/riscv64i-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64 -march=rv64ic asm.S -o bin/$crate.o
ar crs bin/riscv64ic-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=ilp32f -march=rv32if asm.S -o bin/$crate.o
ar crs bin/riscv32if-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=ilp32f -march=rv32ifc asm.S -o bin/$crate.o
ar crs bin/riscv32ifc-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64f -march=rv64if asm.S -o bin/$crate.o
ar crs bin/riscv64if-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64f -march=rv64ifc asm.S -o bin/$crate.o
ar crs bin/riscv64ifc-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=ilp32d -march=rv32ifd asm.S -o bin/$crate.o
ar crs bin/riscv32ifd-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=ilp32d -march=rv32ifdc asm.S -o bin/$crate.o
ar crs bin/riscv32ifdc-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64d -march=rv64ifd asm.S -o bin/$crate.o
ar crs bin/riscv64ifd-unknown-none-elf.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64d -march=rv64ifdc asm.S -o bin/$crate.o
ar crs bin/riscv64ifdc-unknown-none-elf.a bin/$crate.o

rm bin/$crate.o
