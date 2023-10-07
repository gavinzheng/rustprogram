fn main() {
  enum Continent { 	// 枚举类型名Continent， 其下列的分支变量都是不带数据的
    Europe, 		// 分支变量1 - 整数值为0u8 
    Asia,			// 分支变量2 - 整数值为1u8  
    Africa, 		// 分支变量3 - 整数值为2u8
    America, 		// 分支变量4 - 整数值为3u8
    Oceania, 		// 分支变量5 - 整数值为4u8 
    }
    let contin = Continent::Asia; 
    match contin { 							// 使用match语句匹配模式来使用枚举变量的值
      Continent::Europe => println!("Europe"), 
      Continent::Asia => println!("Asia"), 
      Continent::Africa => println!("Afric"), 
      Continent::America => println!("America"), 
      Continent::Oceania => println!("Oceania"), 
    }
    println!("{} ", contin as usize);
}