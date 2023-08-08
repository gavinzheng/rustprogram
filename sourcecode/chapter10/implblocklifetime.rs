#[derive(Debug)] 
struct Coord<'a> { 
  x: &'a u8 
}
impl<'a> Coord<'a> { 
  fn getx(&self) -> &'a u8 { 
    self.x
  } 
  fn setx(&mut self, xcoord: &'a u8) { 
    self.x= xcoord 
  } 
} 
fn main() { 
  let xval = 100; 
  let mut xpoint= Coord{ x: &xval  }; 
  xpoint.setx(&56); 
  println!("{:?}", xpoint.getx()); 
} 