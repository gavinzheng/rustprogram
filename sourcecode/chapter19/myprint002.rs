macro_rules! my_print {
  (foo <> $e: expr) => { println!("FOOOOOOOOOOOOoooooooooooooooooooo!! {}", $e); };
  ($e: expr) => { println!("{}", $e); };
  ($i: ident, $e: expr) => {
      let $i = {
          let a = $e;
          println!("{}", a);
          a
      };
  };
}

fn main() {
  my_print!(x, 30 + 12);
  
  my_print!("hello!");

  my_print!(foo <> "hello!");
}