#[repr(u32)] 
enum Tag { I , F } 
#[repr(C)] 
union U { 
  i : i32 , 
  f : f32 , 
}

#[repr(C)] 
struct Value { 
  tag : Tag , 
  u : U, 
}
fn is_zero(v : Value) -> bool { 
  unsafe { 
    match v { 
      Value { tag : Tag::I , u : U {i:0}} => true , 
      Value { tag : Tag::F , u : U {f:0.0 }}=>true , 
      _ => false , 
    }
  }
}

fn main () { 
  let int_0 = Value{tag : Tag ::I, u : U{i : 0}} ; 
  let float_0 =Value{ tag : Tag :: F , u : U{f: 0.0}}; 
  assert_eq!(true , is_zero(int_0 ));
  assert_eq!(true , is_zero(float_0)); 
  assert_eq!(4, std :: mem ::size_of :: <U>( )); 
}