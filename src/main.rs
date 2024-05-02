#![no_std]
#![no_main]

use core::fmt::{self, Write};
core::arch::global_asm!(include_str!("entry.asm"));

use sbi::shutdown;

mod lang_item;
mod sbi;

const SYSCALL_EXIT: usize = 93;//退出指令是93
const SYSCALL_WRITE: usize = 64;//写指令是64

//系统调用的常用格式
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        );
    }
    ret
}

//系统调用退出执行
pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

//系统调用写
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
  }

struct Stdout;

//封装，将字符串转移为u8数组
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
   }
}
  
pub fn print(args: fmt::Arguments) {
  Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
       $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}
  
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
       print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
  

#[no_mangle]
pub fn rust_main() -> ! {
    shutdown();
}

