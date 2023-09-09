use std::cell::UnsafeCell;

fn main() {
    let unsafecell = UnsafeCell::new(10u32);
    println!("value: {}", unsafe { *unsafecell.get() });
}