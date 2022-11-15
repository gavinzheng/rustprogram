fn main(){
  // let var_a = String::from("This is a test");
  // var_a.push('a');
  
  let mut var_a = String::from("This is a test");
  println!("{}",  var_a.len());
  let var_b = &mut var_a;
  // let mut var_b = &var_a;
  var_b.push('b');
 
  println!("var_b {}", var_b);
  println!("var_a {}",  var_a);
  
  // let var_c = &var_b;

  // var_a.push('a');
  // println!("{} {} {}", var_a, var_b, var_c);
  // var_a.push('a');
}