#[derive(Debug, Clone,Copy)]
struct Coord {
  a: i32,
  b: i32,
//  c: String,
}
// impl Clone for Coord {
//   fn clone(&self)->Self{
//     Self{a: self.a, b: self.b}
//   }
// }
#[allow(unused_variables)]
fn main() {
  let var_1 = Coord{a:10,b:20,};	
  //let var_1 =Coord {a:10,b:20,/*c:"test".to_string()*/};
  call_procedure(var_1);
  println!("{:?}", var_1);
}

fn call_procedure(param: Coord) {
  println!("{:?}", param);
}