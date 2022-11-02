use std::mem::size_of; 

fn main(){
	// assert_eq!(size_of::<Ordering>(), 1); 
	// assert_eq!(size_of::<HttpStatus>(), 2); 

	println!("{} {} {} {} {} {} {} {} {} {} {} {}",
			size_of::<i8>(),
			size_of::<u8>(),
			size_of::<i16>(),
			size_of::<u16>(),
			size_of::<i32>(),
			size_of::<u32>(),
			size_of::<i64>(),
			size_of::<u64>(),
			size_of::<f32>(),
			size_of::<f64>(),
			size_of::<bool>(),
			size_of::<char>());

	println!("{} {} {} {}",
			size_of::<isize>(),
			size_of::<usize>(),
			size_of::<&i8>(),
			size_of::<&u32>());		
	
	println!("{}",std::mem::size_of::<*mut u8>()); // 8Byte Raw Pointer
	println!("{}",std::mem::size_of::<*mut [u8]>()); // 16Byte Fat Pointer = 8Byte Pointer + 8Byte length
	println!("{}",std::mem::size_of::<*mut [u8;4]>()); // 8Byte Raw Pointer
	println!("{}",std::mem::size_of::<*mut str>()); // 16Byte Fat Pointer = 8Byte Pointer + 8Byte length
	println!("{}",std::mem::size_of::<*mut dyn Drop>()); // 16Byte Fat Pointer = 8Byte Pointer + 8Byte vTable
}

// fn main(){
// assert_eq!(size_of::<Ordering>(), 1); 
// assert_eq!(size_of::<HttpStatus>(), 2); 
// }