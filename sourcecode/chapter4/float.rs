fn main(){
	let a: f64 = 4.6; 
	let b: f32 = 3.91; 
	print!("{} {}", a, b); 	// 会打印 " 4.6 3.91 "
	assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // exactly 5.0, per IEEE 
	assert_eq!(-1.01f64.floor(), -1.0); 
	assert!((-1. / std::f32::INFINITY).is_sign_negative()); 
}