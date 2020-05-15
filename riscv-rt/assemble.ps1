# remove existing blobs because otherwise this will append object files to the old blobs
Remove-Item -Force bin/*.a

$crate = "riscv-rt"
$extension_sets = @("i", "im", "ic", "imc")

foreach ($ext in $extension_sets) {
    riscv64-unknown-elf-gcc -ggdb3 -c -mabi=ilp32 -march=rv32$ext asm.S -o bin/$crate.o
    riscv64-unknown-elf-ar crs bin/riscv32$ext-unknown-none-elf.a bin/$crate.o

    riscv64-unknown-elf-gcc -ggdb3 -c -mabi=lp64 -march=rv64$ext asm.S -o bin/$crate.o
    riscv64-unknown-elf-ar crs bin/riscv64$ext-unknown-none-elf.a bin/$crate.o
}

Remove-Item bin/$crate.o
