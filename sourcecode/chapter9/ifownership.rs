
fn f(x: Vec<u32>){
  println!("in f(x)");
}

fn g(x: Vec<u32>){
  println!("in g(x)");
}
fn k(x: Vec<u32>){
  println!("in h(x)");
}

fn main(){
  let x = vec![1, 2, 3, 4, 5];
  let c = false;

  if c {
    f(x);
  }else{
    // g(x);
    println!("if c is false, g(x)");
  }
  h(x)
}