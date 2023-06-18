/* It prints:
e*/
fn main() {
    fn print_nth_char(s: &str, mut n: u32) {
        let mut iter: std::str::Chars = s.chars();
        loop {
            let item: Option<char> = iter.next();
            match item {
                Some(c) => if n == 1 { print!("{}", c); print!("Some"); },
                None => { print!("None"); break; },
            }
            n -= 1;
        }
    }
    print_nth_char("测试Rust", 2);
}
