.option norvc
.section .boot, "ax",@progbits
.global _start
.global abort

_start:
    /* Set up stack pointer. */
    lui     sp, %hi(stacks + 1024)
    ori     sp, sp, %lo(stacks + 1024)
    /* Now jump to the rust world; __start_rust.  */
    j       __start_rust

abort:
    j      abort

.bss
    .global stacks
stacks:
    .skip 1024
