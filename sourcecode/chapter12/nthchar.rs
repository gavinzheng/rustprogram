/* It prints:
e*/
fn main() {
    fn print_char_at(str_in: &str, mut pos: u32) {
        let mut iter: std::str::Chars = str_in.chars();
        loop {
            let item: Option<char> = iter.next();
            match item {
                Some(c) => if pos == 1 { print!("{}", c); },
                None => { break; },
            }
            pos -= 1;
        }
    }
    print_char_at("测试Rust", 2);
}
