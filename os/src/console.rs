//! SBI console driver, for text output
use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;
// 基于sbi实现 Write trait 
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // 循环将字符输出。
        for c in s.chars() {
            console_putchar(c as usize);
        }  
        Ok(())
    }
}
/// 实现print函数
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// Print! to the host console using the format string and arguments.
/// 自定义实现的 print! 不用 std中的宏。 
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    }
}

/// Println! to the host console using the format string and arguments.
/// 自定义实现的 println! 不用 std中的宏。 
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}
