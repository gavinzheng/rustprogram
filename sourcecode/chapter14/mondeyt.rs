use std::ops::Add; 
#[derive(Debug)] 
struct Score<T> { 
  len: T, 
  unit: String 
} 
impl<T: Add<T, Output=T>> Add for Score<T> { 
  type Output = Score<T>; 
  fn add(self, rhs: Score<T>) -> Self::Output { 
    assert!(self.unit == rhs.unit); 
    Score { unit: rhs.unit, len: self.len + rhs.len } 
  } 
} 
fn main() { 
  let jump_try_1: Score<f32> = Score { len: 5.6, unit: "Meter".to_string() }; 
  let jump_try_2: Score<f32> = Score { len: 5.6, unit: "Meter".to_string() }; 

  let summed_jump_try = jump_try_1 + jump_try_2; 
  println!("Summed tries: {:?}", summed_jump_try); 
} 