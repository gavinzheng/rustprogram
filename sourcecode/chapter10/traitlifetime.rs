use std::fmt::Debug; // 用于限定的 trait 
#[derive(Debug)] 
struct structhasRef<'a, T: 'a>(&'a T); 
// `structhasRef` 包含一个指向指向泛型类型 `T` 的引用，其中 `T` 拥有 一个未知的生命周期 `'a`。
// `T` 是被限定的，从而在 `T` 中的任何引用都必须比 `'a` 活得更长。另外 `structhasRef` 的生命周期 
//  也不能超出 `'a`。 
fn print<T>(p: T) 
where T: Debug {  // 一个泛型函数，使用 `Debug` trait 来打印内容。
println!("`print`: p is {:?}", p); 
} 
// 这里接受一个指向 `T` 的引用，其中 `T` 实现了 `Debug` trait， 并且在 `T` 中的所有引用都必须比函数存活时间更长。 
fn print_structref<'a, T>(p: &'a T) 
where T: Debug + 'a { 
println!("`print_structref`: p is {:?}", p); 
} 
fn main() { 
let x = 10; 
let ref_x = structhasRef(&x); 
print_structref(&ref_x); 
print(ref_x); 
} 