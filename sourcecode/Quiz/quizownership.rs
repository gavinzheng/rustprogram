
fn main(){
  let mut arr = ['a', 'b', 'c', 'd', 'e', 'f']; 
  for c in arr { 
    println!("{}", c); 
  } 

  arr[0]='z';
  for c in arr { 
    println!("{}", c); 
  } 

  // for item in arr {
  //   if let Some(elt) = item.iter().rev().next() {
  //     println!("{}", elt);
  //   }
  // }
}
