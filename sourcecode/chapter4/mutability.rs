fn main(){
  let number; 				// 默认是不可变的
  number = 12; 				// 此处因为是初始化值，所以OK
  println!("{}", number); 
  // number =100 ;				// 此处会触发编译错误
}