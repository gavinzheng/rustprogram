
struct Dimension {
  x : u32,
  y:  u32,
}

fn main(){
  let (mut a, b) = (20, "test"); 
  println!("{} {}", a, b);

  let (mut a, mut b) = (20, 100); 
  let Dimension { x: ref a, y: ref b} = p; 
}