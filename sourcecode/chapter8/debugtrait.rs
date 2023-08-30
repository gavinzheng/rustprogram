use std::fmt; 
#[derive(Clone)] 	// 内置派生
struct Point { 
  x: u64, 	// x轴坐标
  y: u64, 	// y轴坐标
} 
impl fmt::Debug for Point { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!( f, "Point {{ x: {}, y : {} }}", self.x, self.y ) 
    } 
}
fn main() { 
  let element= Point{ x:100, y:200 }; 
  println!("Point: {:?}", element); 
} 