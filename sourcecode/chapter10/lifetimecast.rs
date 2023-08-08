// 在下面的函数里，Rust 推定了一个尽可能短的生命周期。然后这两个引用都被强制转成这个生命周期。 
fn mul<'a>(outer: &'a i32, inner : &'a i32) -> i32 { // 此处’a等于inner和outer之中较短的生命周期
  outer * inner 
} 
// `<'a: 'b, 'b>` 理解为生命周期 `'a` 至少和 `'b` 一样长。 
// 在这里我们我们接受了一个 `&'a i32` 类型并返回一个 `&'b i32` 类型，这是强制转换得到的结果。 
fn choose_outer<'a: 'b, 'b>(outer: &'a i32, first: &'b i32) -> &'b i32 { 
  first 
} 
fn main() { 
  let outer= 10; // 较长的生命周期 
  { 
    let inner = 20; // 较短的生命周期 
    println!("The product is {}", mul(&outer, &inner )); // mul中’a 等于inner的生命周期
    println!("{} is the outer", choose_outer(&outer, &inner )); 
  }; 
} 