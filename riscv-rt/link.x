/* NOTE: Adapted from cortex-m/link.x */
INCLUDE memory.x

PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

SECTIONS
{
  PROVIDE(_stext = ORIGIN(FLASH));

  .text ALIGN(_stext,4) :
  {
    /* Put reset handler first in .text section so it ends up as the entry */
    /* point of the program. */
    KEEP(*(.init));
    KEEP(*(.init.rust));
    KEEP(*(.trap));
    KEEP(*(.trap.rust));

    *(.text .text.*);
  } > FLASH

  .rodata ALIGN(4) :
  {
    *(.rodata .rodata.*);
  } > FLASH

  PROVIDE(_sbss = ORIGIN(RAM));
  .bss ALIGN(_sbss,4) :
  {
    *(.bss .bss.*);
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  .data _ebss :
  {
    _sidata = LOADADDR(.data);
    _sdata = .;
    /* Must be called __global_pointer$ for linker relaxations to work. */
    PROVIDE(__global_pointer$ = . + 0x800);
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } > RAM AT > FLASH /* LLD fails on AT > FLASH */

  PROVIDE(_heap_size = 0);

  /* fictitious region that represents the memory available for the heap */
  .heap _edata (INFO) : ALIGN(4)
  {
    _sheap = .;
    . += _heap_size;
    . = ALIGN(4);
    _eheap = .;
  }

  /* fictitious region that represents the memory available for the stack */
  .stack _eheap (INFO) : ALIGN(4)
  {
    _estack = .;
    . = _stack_start;
    _sstack = .;
  }

  /* fake output .got section */
  /* Dynamic relocations are unsupported. This section is only used to detect
     relocatable code in the input files and raise an error if relocatable code
     is found */
  .got :
  {
    _sgot = .;
    KEEP(*(.got .got.*));
    _egot = .;
  } > RAM AT > FLASH /* LLD fails on AT > FLASH */

  /* Discard .eh_frame, we are not doing unwind on panic so it is not needed */
  /DISCARD/ :
  {
    *(.eh_frame);
  }
}

/* Do not exceed this mark in the error messages below                | */
ASSERT(_sgot == _egot, "
.got section detected in the input files. Dynamic relocations are not
supported. If you are linking to C code compiled using the `gcc` crate
then modify your build script to compile the C code _without_ the
-fPIC flag. See the documentation of the `gcc::Config.fpic` method for
details.");
