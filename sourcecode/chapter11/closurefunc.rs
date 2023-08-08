/* It prints:
30 30 30 30 30 */
fn main() {
    let factor = 10;
    let multiply = |a| a * factor;
    print!("{}", multiply(13));
    let multiply_ref: &(Fn(i32) -> i32) = &multiply;
    print!(
        " {} {} {} {} {}",
        (*multiply_ref)(3),
        multiply_ref(3),
        (|a| a * factor)(3),
        (|a: i32| a * factor)(3),
        |a| -> i32 { a * factor }(3));
}
