fn main(){
  // let var_a = String::from("This is a test");
  let mut var_a = String::from("This is a test");	
  // var_a.push('a');	

  let var_b = &var_a;
  // let var_b = &var_a;
  //var_b.push('b');
 
  let var_c = &var_b;

  var_a.push('a');
  println!("{} {} {}", var_a, var_b, var_c);
  // var_a.push('a');
}