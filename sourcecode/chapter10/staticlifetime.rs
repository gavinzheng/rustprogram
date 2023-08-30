use std::{slice::from_raw_parts, str::from_utf8_unchecked};

fn get_memory_location() -> (usize, usize) {
  
  // 但持有它的变量 `localstring` 的生命周期就不一样了，它完全取决于变量作用域，对于该例子来说，也就是当前的函数范围
  let localstring  = "Hello Milly!";    // “Hello World” 是字符串字面量，因此它的生命周期是 `'static`.
  let pointer = localstring .as_ptr() as usize;
  let length = localstring .len();
  (pointer, length)
  // `string` 在这里被 drop 释放
  // 虽然变量被释放，无法再被访问，但是数据依然还会继续存活
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
  // 使用裸指针需要 `unsafe{}` 语句块
  unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

fn main() {
  let (pointer, length) = get_memory_location();
  let message = get_str_at_location(pointer, length);
  println!(
    "The {} bytes at 0x{:X} stored: {}",
    length, pointer, message
  );
  // 如果想知道为何处理裸指针需要 `unsafe`，可以去掉以下代码的注释看看
  // let message = get_str_at_location(1000, 10);
}
