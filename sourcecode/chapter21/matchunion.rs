#[repr(C)]
union MyUnion{ 
  mem_f: f32, 
  mem_i: i8, 
}

fn main() {
    let float_num = MyUnion { mem_f: 10.0 };
    let f = unsafe { float_num.mem_f };
    println!("f is {:.3}", f);

    unsafe { 
      match float_num { 
        MyUnion{ mem_i: 1 } => { println!("i32 value 1"); } 
        MyUnion{ mem_f: 10.0  } => { println!("float 10.0"); } 
        _ => { println!("其他"); } 
      } 
    } 

    // float_num.mem_i= 100 ;

    unsafe {
      match float_num {
        MyUnion { mem_f } => { println!("float {}", mem_f) },
        // warning: unreachable pattern
        MyUnion { mem_i } => { println!("int {}", mem_i) }
      }
     }
}