New-Item -Force -Name bin -Type Directory

# remove existing blobs because otherwise this will append object files to the old blobs
Remove-Item -Force bin/*.a

$crate = "riscv"

riscv64-unknown-elf-gcc -c -mabi=ilp32 -march=rv32i asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv32i.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=ilp32 -march=rv32ic asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv32ic.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64 -march=rv64i asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv64i.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64 -march=rv64ic asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv64ic.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=ilp32f -march=rv32if asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv32if.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=ilp32f -march=rv32ifc asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv32ifc.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64f -march=rv64if asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv64if.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64f -march=rv64ifc asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv64ifc.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=ilp32d -march=rv32ifd asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv32ifd.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=ilp32d -march=rv32ifdc asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv32ifdc.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64d -march=rv64ifd asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv64ifd.a bin/$crate.o

riscv64-unknown-elf-gcc -c -mabi=lp64d -march=rv64ifdc asm.S -o bin/$crate.o
riscv64-unknown-elf-ar crs bin/riscv64ifdc.a bin/$crate.o

Remove-Item bin/$crate.o
