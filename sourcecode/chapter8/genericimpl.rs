struct GenericVal<T>(T,); // 泛型类型 `GenericVal` 

// `<T>` 必须在类型之前给出来以保持泛型。 
impl <T> GenericVal<T> {} 
struct Val { 
val: f64 
} 
struct GenVal<T>{ 	// 泛型结构
gen_val: T 
} 
// Val 的实现(impl) 
impl Val { 
fn value(&self) -> &f64 { &self.val } 
} 
// GenVal 针对泛型类型 `T` 的实现 
impl <T> GenVal<T> {  // 对泛型结构需要impl块也使用泛型
fn value(&self) -> &T { &self.gen_val } 
} 
fn main() { 
let x = Val { val: 3.0 }; 
let y = GenVal { gen_val: 3i32 }; 
println!("{}, {}", x.value(), y.value()); 
} 