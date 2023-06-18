fn main(){
  let arr = [10u8, 14, 5, 76, 84, 35, 23, 94, 100, 143, 23, 200, 12, 94, 72]; 
  // 遍历arr，克隆最后生成新的向量
  let collection: Vec<_> = arr.iter().skip(2).take(8).cloned().collect();  // 没有指定向量成员类型
  for elm in collection { 
    println!("{}", elm); 
  }
}
