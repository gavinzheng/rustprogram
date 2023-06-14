fn main() { 
    let a = &vec![1]; 		// vec的作⽤域延⻓到所在区块 
    let b = Some(&vec![2]);  // 发⽣函数调⽤，作⽤域限制在本语句 
    println!("{:?}", a);     //  OK
    println!("{:?}", b); 	// 错误，vec![2]已经被释放
} 