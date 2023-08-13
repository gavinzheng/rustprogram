use std::fs::File; 
use std::error::Error; 
use std::net::Ipv6Addr; 

fn main() -> Result<(), Box<dyn Error>> { 	//该函数返回std::io::Error类型
    let _f = File::open("nonexisting.txt")?; // File::Open返回Result<(),std::io.Error>
    // parse::<Ipv6Addr>返回:Result<Ipv6Addr, std::net.AddrParseError>
    let _localhost = "::1".parse::<Ipv6Addr>()? ;	

    Ok(()) 
}