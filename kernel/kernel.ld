ENTRY(_start)

SECTIONS {
  . = 0x100000; /* Kernel load address */

  .text : {
    *(.multiboot) /* optional */
    *(.text*)
  }

  .rodata : {
    *(.rodata*)
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
