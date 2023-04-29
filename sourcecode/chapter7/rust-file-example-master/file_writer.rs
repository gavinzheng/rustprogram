use std::io::fs::File;
use std::io::{Write, Truncate};

fn main() {
  let f =
      File::open_mode(&Path::new("./test.dat"), Truncate, Write);

  let num0 : u64 = 90;
  let num1 : u64 = 22;

  let mut f = f.expect("open_mode failed.");

  f.write_be_u64(num0);
  f.write_be_u64(num1)
}
