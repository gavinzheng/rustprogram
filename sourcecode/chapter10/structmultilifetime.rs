struct Coord<'a> { 
  x: &'a i32, 
  y: &'a i32 
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
}
