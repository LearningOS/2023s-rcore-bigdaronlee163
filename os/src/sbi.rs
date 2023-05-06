//! SBI call wrappers

use core::arch::asm;

const SBI_CONSOLE_PUTCHAR: usize = 1;

/// general sbi call
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    /*
    1. 接收 SBI 接口编号 which，以及对应的三个参数 arg0、arg1 和 arg2。在函数内部声明一个变量 ret，该变量用于保存 SBI 接口的返回值。
    2. 使用 Rust 的 asm! 宏定义了一段汇编代码，通过 ecall 指令来触发系统调用。在这个汇编片段中，要注意以下几点：
        li x16, 0 在汇编层面设置了返回值寄存器 x16 的初值为 0，这意味着在调用 SBI 接口时，如果没有正确处理返回值的话，那么默认情况下就会返回 0。
        inlateout("x10") arg0 => ret 确保了参数 arg0 和返回值 ret 使用同一个寄存器 x10，这是因为 SBI 接口的实现中可能会修改原始参数，而我们需要将修改后的返回值传递回 Rust 函数中。
        in("x11") arg1、in("x12") arg2 以及 in("x17") which 将其他三个参数和 SBI 接口编号传入到相应的寄存器中，供系统调用使用。 

    SBI 接口调用完成后，将返回值保存到变量 ret 中，并最终返回给调用方。
    需要注意的是，这个函数使用了 Rust 的 unsafe 关键字，因为它直接嵌入了汇编代码，绕过了 Rust 的类型检查和内存安全机制。如果汇编代码中存在错误或漏洞，那么就有可能导致程序崩溃、损坏数据或甚至造成安全漏洞。因此，在使用 unsafe 代码时必须格外小心，确保其正确性和安全性。
     */
    unsafe {
        asm!(
            "li x16, 0",
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}

/// use sbi call to putchar in console (qemu uart handler)
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

use crate::board::QEMUExit;
/// use sbi call to shutdown the kernel
pub fn shutdown() -> ! {
    crate::board::QEMU_EXIT_HANDLE.exit_failure();
}
