fn compose<A,B,C,F,G>(f: F, g: G) -> impl Fn(A) -> C 
  where F: 'static + Fn(A) -> B, 
        G: 'static + Fn(B) -> C { 
  move |x| g(f(x)) 
} 
fn main() { 
  let fa = |x| x+10; 
  let fb = |y| y*4; 
  let fc = |z| z/10; 
  let g = compose(compose(fa,fb),fc); 
  println!("g(10) = {}", g(1)); 
  println!("g(20) = {}", g(12)); 
  println!("g(120) = {}", g(120)); 
}