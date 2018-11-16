OUTPUT_ARCH("riscv")

ENTRY(_start)

SECTIONS
{
    . = 0x80000000;

    /* text: Program code section */
    .text :
    {
        *(.boot)
        *(.text)
        *(.text.*)
    }

    /* rodata: Read-only data */
    .rodata :
    {
        *(.rdata)
        *(.rodata)
        *(.rodata.*)
    }

    /* data: Writable data */
    .data :
    {
        *(.data)
        *(.data.*)
    }

    . = .;
    _bss_start = .;
    .bss :
    {
        *(.bss)
        *(.bss.*)
    }
}