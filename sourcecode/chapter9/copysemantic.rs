fn main(){
  let a : i32 = 100;	// 变脸a类型是i32，实现了Copy Trait
  let b = a	;		// 此处发生拷贝语义
  println!("{} {}", a, b);
  }