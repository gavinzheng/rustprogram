struct S<'a> { 
  r: &'a i32 	// 会触发编译器错误
}
fn main() { 
  let s; 
  { 
      let x = 10; 
      s = S { r: &x }; 
  } 
  assert_eq!(*s.r, 10); // s.r是指向x的引用，而x已经被释放，所以触发错误 
}