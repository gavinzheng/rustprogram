fn main(){
	let one_thousand = 1e3; 
	let one_million = 1e6; 		// 类型推定i32类型-1000_000
	let twentyfive_billions_and_half = 25.5e9; 
	let fiftysix_millionths = 56e-6; 

	print!("one_thousand= {} one_million = {} twentyfive_billions_and_half= {} fiftysix_millionths={}", one_thousand, one_million, twentyfive_billions_and_half, fiftysix_millionths );
	
	assert_eq!(2u16.pow(4), 16);  
	assert_eq!((-4i32).abs(), 4);
}