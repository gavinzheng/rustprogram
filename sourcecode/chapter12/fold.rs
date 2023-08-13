fn main() {
  let arr = [1, 2, 3]; 
  let sum = arr.iter()
              .	fold(0, |acc, elem| acc + elem);// acc就是累计变量，初始值为0，保存本轮迭代的中间值
  println!("sum {}", sum); 
  for elm in &arr { 	// 切片实现了IntoIterator trait，而数组不支持Iterator
    println!("{}", elm); 
  } 
}