#![no_std]
#![no_main]
#![feature(alloc)]

extern crate alloc;

#[macro_use]
extern crate rust;

use alloc::vec::Vec;
use core::ptr;

use rust::io::get_line;
use rust::syscall::sys_exec;

// IMPORTANT: Must define main() like this
#[no_mangle]
pub fn main() -> i32 {
    println!("Rust user shell");
    let mut history = Vec::new();

    loop {
        print!(">> ");
        let cmd = get_line(&mut history);
        // split cmd, make argc & argv
        // to-do: handle quotes
        print!("{}???", cmd);
        // let cmd = cmd.replace(' ', "\0") + "\0";
        // let cmds: Vec<&str> = cmd.split('\0').collect();
        // let mut ptrs: Vec<usize> = cmd
        //     .split('\0')
        //     .filter(|s| !s.is_empty())
        //     .map(|s| s.as_ptr() as usize)
        //     .collect();
        // if ptrs.is_empty() {
        //     continue;
        // }
        // sys_exec(ptrs[0] as *const u8);
    }
}
