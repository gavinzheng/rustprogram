
use   std::arch::asm;

fn subtract(a: i32) -> i32 { 
  //let sub: i32; 
  let mut x: i32 = a;
  unsafe { 
    // asm!("sub $2, $1; mov $1, $0" 
    // : "=r"(sub) 
    // : "r"(a), "r"(b) 
    // ); 
    // asm!("sub $2, $1; mov $1, $0" : "=r"(sub) : "r"(a), "r"(b));
      asm!(
        "mov {tmp}, {x}",
        "shl {tmp}, 1",
        "shl {x}, 2",
        "add {x}, {tmp}",
        x = inout(reg) x,
        tmp = out(reg) _,
    );
  } 
  x //sub 
} 

fn main() { 
  println!("{}", subtract(42, 7)) 
} 
