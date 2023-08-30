macro_rules! add_to_vec { 
  ($( $x:expr; [ $( $y:expr ),* ]);* ) => { 
  &[ $($( $x + $y ),*),* ] 
  } 
  }
  fn main() {
    println!("add_to_vec : {:?}", add_to_vec!(11; [20, 30]; 22; [50, 60]));
  }