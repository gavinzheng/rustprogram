#[derive(Default)]	// 实现Default trait
enum Candidate{
    #[default]		// 此枚举类型的默认值为Alice
    Alice,
    Bob,
    Milly,
}
fn main() {
    let options: Candidate= Default::default();	// 调用default方法
}