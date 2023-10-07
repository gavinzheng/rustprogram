use std::any::{Any , TypeId} ; 

enum Season { Autumn, Summer , Spring, Winter} 
struct Point { x : u16 , y : u16 } 

fn main () { 
  let valuel = 20000 as u32; 
  let value2 = Season::Autumn ; 
  let value3 = Point { x : 1 , y : 2}; 
  let value4 = "rust"; 
  let mut av : &dyn Any ; 
  av= &valuel ; 
  assert!(av.is::<u32> ()) ; 
  println!("{:?}", TypeId::of::<u32>( )); 

  av = &value2 ; 
  assert!(av.is::<Season> ()); 
  println!("{:?}", TypeId::of::<Season>()) ; 
  av = &value3 ; 
  assert! (av.is::<Point> ()) ; 

  println!("{:?}", TypeId::of::<Point>()) ; 
  av= &value4 ; 
  assert!(av.is::<&str> ()); 
  println!("{:?}",  TypeId::of::<&str>() ); 
}