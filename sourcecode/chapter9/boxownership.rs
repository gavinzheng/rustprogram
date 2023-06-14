fn main(){ 
    let gpa = Box::new((88, 96)); 		// 此处分配gpa的堆内存
    let output = format!("{:?}", gpa); 	// 此处分配output的堆内存 
    println!("{:?}", output);
} // gpa和output离开其作用域，析构