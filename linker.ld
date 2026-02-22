ENTRY(_start)

SECTIONS
{
    /* Tell the linker to start placing code at 0x80000000 */
    . = 0x80000000;

    .text : {
        /* Ensure the entry point is the very first thing in the binary */
        *(.text._start)
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
    } > ram 
    __stack_top = . + 0x20000;
    _heap_start = ALIGN(4096);
    
}
