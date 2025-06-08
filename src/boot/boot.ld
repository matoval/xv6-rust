OUTPUT_FORMAT("binary")
ENTRY(start)

SECTIONS {
  . = 0x7C00;
  .text : { *(.text*) }
  .rodata : { *(.rodata*) }
  .data : { *(.data*) }
  .bss : { *(.bss*) *(COMMON) }
}

ENTRY(_start)

SECTIONS {
  . = 0x100000; /* Kernel load address */

  .text : {
    *(.multiboot) /* Multiboot header section */
    *(.text*)
  }

  .data : {
    *(.data*)
  }

  .bss : {
    *(.bss*)
    *(COMMON)
  }

  .stack : {
    *(.stack)
  }
}

