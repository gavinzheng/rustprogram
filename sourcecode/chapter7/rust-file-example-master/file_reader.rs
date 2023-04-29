use std::io::fs::File;
use std::io::{Open, Read, SeekEnd, SeekSet};
use std::mem::size_of;

fn main() {
    let f = File::open_mode(&Path::new("./test.dat"), Open, Read);

    let mut f = f.expect("open_mode failed.");

    f.seek(0, SeekEnd);
    if f.tell() < (2 * size_of::<u64>()) as u64 {
      fail!()
    }
    f.seek(0, SeekSet);
    let num0 = f.read_be_u64();
    let num1 = f.read_be_u64();
    println(num0.to_str());
    println(num1.to_str())
}
