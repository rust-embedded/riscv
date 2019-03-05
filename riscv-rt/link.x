/* NOTE: Adapted from cortex-m/link.x */
INCLUDE memory.x

PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

PROVIDE(trap_handler = default_trap_handler);

/* # Pre-initialization function */
/* If the user overrides this using the `#[pre_init]` attribute or by creating a `__pre_init` function,
   then the function this points to will be called before the RAM is initialized. */
PROVIDE(__pre_init = default_pre_init);

SECTIONS
{
  PROVIDE(_stext = ORIGIN(FLASH));

  .text ALIGN(_stext,4) :
  {
    /* Put reset handler first in .text section so it ends up as the entry */
    /* point of the program. */
    KEEP(*(.init));
    KEEP(*(.init.rust));
    . = ALIGN(4);
    KEEP(*(.trap));
    KEEP(*(.trap.rust));

    *(.text .text.*);
  } > FLASH

  .rodata ALIGN(4) :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .bss :
  {
    _sbss = .;
    *(.bss .bss.*);
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  .data : AT(LOADADDR(.rodata) + SIZEOF(.rodata))
  {
    _sidata = LOADADDR(.data);
    _sdata = .;
    /* Must be called __global_pointer$ for linker relaxations to work. */
    PROVIDE(__global_pointer$ = . + 0x800);
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } > RAM

  PROVIDE(_heap_size = 0);

  /* fictitious region that represents the memory available for the heap */
  .heap (INFO) :
  {
    _sheap = .;
    . += _heap_size;
    . = ALIGN(4);
    _eheap = .;
  } > RAM

  /* fictitious region that represents the memory available for the stack */
  .stack (INFO) :
  {
    _estack = .;
    . = _stack_start;
    _sstack = .;
  } > RAM

  /* fake output .got section */
  /* Dynamic relocations are unsupported. This section is only used to detect
     relocatable code in the input files and raise an error if relocatable code
     is found */
  .got (INFO) :
  {
    KEEP(*(.got .got.*));
  }

  /* Discard .eh_frame, we are not doing unwind on panic so it is not needed */
  /DISCARD/ :
  {
    *(.eh_frame);
  }
}

/* Do not exceed this mark in the error messages below                | */
ASSERT(SIZEOF(.got) == 0, "
.got section detected in the input files. Dynamic relocations are not
supported. If you are linking to C code compiled using the `gcc` crate
then modify your build script to compile the C code _without_ the
-fPIC flag. See the documentation of the `gcc::Config.fpic` method for
details.");
