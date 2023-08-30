struct Coord{ x: i32, y: i32 } 

fn f(x: &str) -> &str { 
  println!("x is : {}", x);
  x 
} 
fn main() { 
  let coord = Coord{ x: 100, y: 160 }; 
  let r1: &Coord= &coord ; 	// 1重引用
  let r2: &&Coord= &r1; 		// 2重引用
  let r3: &&&Coord= &r2; 		// 3 重引用
  f(&&&&&&&"It's a string"); // 编译器会对&&&&&&&str强制解引用为&str，并传入函数f()  
  assert_eq!(r3.y, 160);
} 