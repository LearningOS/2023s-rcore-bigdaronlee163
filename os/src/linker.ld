OUTPUT_ARCH(riscv)
ENTRY(_start) 
BASE_ADDRESS = 0x80200000;  /* riscv程序的入口地址 */

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;  /* 符号赋值语句 */

    stext = .;
    .text : {    /*一个特殊的输出 section 关键字*/
        *(.text.entry)  /*一个输入section描述*/
        *(.text .text.*)  /*直接包含的数据值*/
    }

    . = ALIGN(4K);  // 按照4K来对齐
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}