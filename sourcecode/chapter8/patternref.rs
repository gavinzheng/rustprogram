fn main() { 
  let val = 100; 
  match val  { 
    ref r => println!("不可变引用 {}", r), // 此处r是val 的共享/不可变引用
  } 
  let mut mv = 100; 
  match mv { 
    ref mut mr => { 		 // 此处mr是mv的可变引用
      println!("可变引用 {}", mr); 
      *mr = 200; 
    }, 
  } 
  println!("mv 被修改为 {}!", mv); 
}