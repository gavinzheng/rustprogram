#![feature(non_ascii_idents)]
#[derive(Debug)]
struct 联系人 {
    姓名: String,
    电话: String,
}
impl 联系人 {
    fn 构建新的联系人<T: Into<String>>(新的人姓名: T, 新的人电话: T) -> Self {
        联系人 {
            姓名: 新的人姓名.into(),
            电话: 新的人电话.into(),
        }
    }
}
fn main() {
    let mut 联系人列表: Vec<联系人> = Vec::new();
    联系人列表.push(联系人::构建新的联系人("瑾瑜", "156********"));
    println!("{:?}", 联系人列表[0]);
}