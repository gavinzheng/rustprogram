macro_rules! my_print {
  ($i: ident, $e: expr) => {
      let $i = {
          let a = $e;
          println!("{}", a);
          a
      };
  }
}

fn main() {
  my_print!(x, 30 + 12);
  println!("and again {}", x);
}