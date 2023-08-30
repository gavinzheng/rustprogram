fn test4<'a,'b>(x:&'a i32, y:&'b i32, z:&'b i32)->(&'a i32, &'b i32) {
  let u = if x<y {x} else {y};
  let v = if y<z {y} else {z};
  (u,v)
}
  
fn main() {
  test4(&23, &6, &0);
}