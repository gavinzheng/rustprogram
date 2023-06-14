
fn main() {
    let mut arr = [43, 18, 10, 13, 0, 56, 22, 37];
    use std::cmp::Ordering;
    let desc = |a: &i32, b: &i32| -> Ordering {
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal }
    };
    arr.sort_by(desc);
    print!("{:?}", arr);
}
