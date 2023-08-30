
use   std::arch::asm;

fn subtract(a: i32) -> i32 { 

  let mut x: i32 = a;
  unsafe { 
      asm!(
        "mov {tmp}, {x}",
        "shl {tmp}, 1",
        "shl {x}, 2",
        "add {x}, {tmp}",
        x = inout(reg) x,
        tmp = out(reg) _,
    );
  } 
  x 
} 

fn main() { 
  println!("{}", subtract(56)) 
} 
