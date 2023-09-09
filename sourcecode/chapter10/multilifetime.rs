#![allow(unused)]
fn longest<'a,'b>(x: &'a str, y: &'b str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
}
fn main() {
    let string1 = String::from("北科信链");
    let result;
    {
        let string2 = String::from("瑾瑜");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}