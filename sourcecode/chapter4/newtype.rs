fn main() {
  struct Kilograms(u32); 
  struct Pound(u32);

  let mut weight = Kilograms(250); 		  // weight是一个新的类型
  let Kilograms(kgm) = weight; 		      // 解构kgm 
  println!("weight is {} kilograms", kgm); 

  let weightpound = Pound(100); 		    // weightpound是一个新的类型
  println!("weightfemale is {} pound", weightpound.0); 

  weight = weightpound;
  println!("weight is {} kilograms", weight.0); 
}