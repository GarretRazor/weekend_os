ENTRY(_start)

MEMORY
{
   ram (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
}
SECTIONS
{
    /* Tell the linker to start placing code at 0x80000000 */
    . = 0x80000000;

    .text : {
        /* Ensure the entry point is the very first thing in the binary */
        *(.text._start)
        *(.text .text.*)
    } > ram

    .rodata : {
        *(.rodata .rodata.*)
    } > ram

    .data : {
        *(.data .data.*)
    } > ram

    .bss : {
        *(.bss .bss.*)
    } > ram 
    __stack_top = . + 0x20000;
    _heap_start = ALIGN(4096);
    
}
