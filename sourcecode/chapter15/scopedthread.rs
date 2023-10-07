use thread::*;
fn main() {
  let numbers = vec![1, 2, 3];

  thread::scope(|s| { ①
      s.spawn(|| { ②
          println!("length: {}", numbers.len());
      });
      s.spawn(|| { ②
          for n in &numbers {
              println!("{n}");
          }
      });
  });③
}