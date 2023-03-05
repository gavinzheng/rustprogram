#![feature(associated_consts)]

enum Dimension{ 
    OneD, 
    TwoD,
    ThreeD
  }
  pub struct Coord{ 
    x: f32, 
    y: f32, 
  }
  impl Dimension{ 
    const DEFAULT_DIMENSION : u32 = 1 ; // Dimension::TwoD; 
  } 
  impl Coord{ 
    const ZERO: Coord= Coord{ x: 0.0, y: 0.0 }; 
    const UNIT: Coord= Coord{ x: 1.0, y: 0.0 }; 
  } 
  fn main() { 
  }