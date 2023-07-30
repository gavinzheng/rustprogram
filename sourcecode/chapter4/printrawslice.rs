fn raw_slice(arr: &[i32]) {
  unsafe {
  let (val1, val2) : (usize, usize) = std::mem::transmute(arr);
  println!("Value in raw pointer:");
  println!("value1: {:x}", val1);
  println!("value2: {:x}", val2);
  }
 }
 fn main() {
  let arr : [i32; 5] = [1, 2, 3, 4, 5];
  let address : &[i32; 5] = &arr;
  println!("Address of arr: {:p}", address);
  raw_slice(address as &[i32]);
 }