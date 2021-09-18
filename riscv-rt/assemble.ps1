New-Item -Force -Name bin -Type Directory

# remove existing blobs because otherwise this will append object files to the old blobs
Remove-Item -Force bin/*.a

$crate = "riscv-rt"

$extension_sets = @("i", "im", "ic", "imc", "if", "ifc", "imf", "imfc", "ifd", "ifdc", "imfd", "imfdc")

$pwd = Get-Location

foreach ($ext in $extension_sets)
{
    $abi = ""
    if ($ext.contains("d"))
        {$abi = "d"}
    elseif ($ext.contains("f"))
        {$abi = "f"}

    riscv64-unknown-elf-gcc -ggdb3 -fdebug-prefix-map=$pwd=/riscv-rt -c "-mabi=ilp32$abi" "-march=rv32$ext" asm.S -o bin/$crate.o
    riscv64-unknown-elf-ar crs bin/riscv32$ext-unknown-none-elf.a bin/$crate.o

    riscv64-unknown-elf-gcc -ggdb3 -fdebug-prefix-map=$pwd=/riscv-rt -c "-mabi=lp64$abi" "-march=rv64$ext" asm.S -o bin/$crate.o
    riscv64-unknown-elf-ar crs bin/riscv64$ext-unknown-none-elf.a bin/$crate.o
}

Remove-Item bin/$crate.o
