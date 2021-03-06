mod structs;
mod scheduler;
mod processor;
mod thread_pool;

use structs::Thread;
use alloc::{ boxed::Box, vec::Vec };
use processor::Processor;
use thread_pool::ThreadPool;
use self::scheduler::Scheduler;
use crate::fs::{ ROOT_INODE, INodeExt };

pub type Tid = usize;
pub type ExitCode = usize;

static CPU: Processor = Processor::new();

pub fn tick() {
    CPU.tick();
}

pub fn exit(code: usize) {
    CPU.exit(code);
}

pub fn kmain() {
    CPU.run();
}

pub fn yield_now() {
    CPU.yield_now();
}

pub fn wake_up(tid : Tid) {
    CPU.wake_up(tid);
}

pub fn current_tid() -> usize {
    CPU.current_tid()
}

pub fn init() {
    println!("+------ now to initialize process ------+");
    let scheduler = Scheduler::new(1);
    let thread_pool = ThreadPool::new(100, scheduler);
    CPU.init(Thread::new_idle(), Box::new(thread_pool));
    println!("???");
    let data = ROOT_INODE
        .lookup("rust/sh")
        .unwrap()
        .read_as_vec()
        .unwrap();
    println!("size of program {:#x}", data.len());
    let user = unsafe{ Thread::new_user(data.as_slice()) };

    CPU.add_thread(user);
}

#[no_mangle]
pub extern "C" fn hello_thread(arg: usize) -> ! {
    println!("hello thread");
    println!("arg is {}", arg);
    for i in 0..100 {
        println!("{}{}{}{}{}{}{}{}", arg, arg, arg, arg, arg, arg, arg, arg);
        for j in 0..1000 {
        }
    }
    println!("end of thread {}", arg);
    CPU.exit(0)
}
