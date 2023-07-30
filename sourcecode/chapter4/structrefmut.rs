struct Struct1 { 
  f: i32 
} 
struct Struct2 <'a> { 
  f: &'a mut Struct1  // 可变的引用字段
} 
struct Struct3 <'a> { 
  f: &'a Struct1,  	// 不可变的引用字段
  v: i32  
} 
fn main() { 
  let mut s1 = Struct1 {f:56}; 
  let s2 = Struct2  { f: &mut s1}; 
  s2.f.f = 100; 			// 合法，尽管s2不可变 
  // s2.f = &mut s1; 		// 不合法 - s2 是不可变的 
  let s1 = Struct1 {f:200}; 
  let mut s3 = Struct3 { f: &s1, v:1}; 
  s3.f = &s1; 			// 合法 - s3是可变的 
  // s3.f.f = 100; 		// 不合法 - s3.f 是不可变的 
} 