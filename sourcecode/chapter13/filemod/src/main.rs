// chapter7/filemod.rs
mod sports;

use sports::Football; 
// use crate::sports::football;	// OK crate为根
// use self::sports::football;	// OK 因为mod和main在同一个文件

fn main() { // 主程序
    let _playing = Football::manifest(); 
}