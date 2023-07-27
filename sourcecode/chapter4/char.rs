fn main(){
	assert_eq!('*'.is_alphabetic(), false); 
	assert_eq!('β'.is_alphabetic(), true); 
	assert_eq!('8'.to_digit(10), Some(8)); 
	assert_eq!('ま'.len_utf8(), 3); // 不是1
	assert_eq!(std::char::from_digit(2, 10), Some('2'));  // Rust专门处理字符的库std::char
}