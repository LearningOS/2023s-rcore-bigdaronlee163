    .section .text.entry
    .globl _start ; 设置_start 入口函数。
_start:
    la sp, boot_stack_top  ; 创建函数栈
    call rust_main ; 调用主函数

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:  // 栈底
    .space 4096 * 16  ; 设置栈空间
    .globl boot_stack_top ;  // 到处栈顶
boot_stack_top:  // 设置栈顶地址