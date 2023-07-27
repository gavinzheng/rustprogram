fn main() {
let big_tuple = (10u8, 20u16, 30u32, 40u64, -10i8, -20i16, -30i32, -40i64,0.1f32, 0.2f64,'z', true); 
// 通过元组的索引来访问具体的值
println!("big tuple 1st value: {}", big_tuple.0); 	// 0是元组第一个域
println!("big tuple 2nd value: {}", big_tuple.1); 	// 1是元组第二个域
}