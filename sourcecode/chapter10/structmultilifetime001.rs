struct Coord<'a, 'b> { 
  x: &'a i32, 
  y: &'b i32 
} 

fn main() {
  let xcoord = 10; 
  let v; 
  { 
    let ycoord = 200; 
    { 
      let s = Coord { x: &xcoord , y: &ycoord  }; 
      v = s.x; 
    } 
  } 
  println!("{}", v);
  // println!("{:?}", s);
}
