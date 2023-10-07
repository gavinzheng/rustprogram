use std::net::TcpStream; 

fn main() {
  TcpStream::connect("192.168.0.1:6000"); 	// 参数是&str类型. 
  let addr_string = "192.168.0.1:6000".to_string(); 	// String 类型
  TcpStream::connect(&addr_string); 		// 将 `addr_string` 转换成 &str. 
}