#![feature(asm)]
use std::arch::asm;
const SYS_WRITE: usize = 1;
const STDOUT: usize = 1;
static MESSAGE: &str = "hello Milly\n";

unsafe fn syscall3(scnum: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let ret: usize;
    asm!(
        "syscall",
        in("rax") scnum,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
        options(nostack),
    );
    ret
}
fn main() {
    unsafe {
        syscall3(
        SYS_WRITE,
        STDOUT,
        MESSAGE.as_ptr() as usize,
        MESSAGE.len() as usize,
        );
    };
}
