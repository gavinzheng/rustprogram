
fn main() {
    let mut arr = [43, 18, 10, 13, 0, 56, 22, 37];
    use std::cmp::Ordering;
    arr.sort_by(|a, b|
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal });
    print!("{:?}", arr);
}
