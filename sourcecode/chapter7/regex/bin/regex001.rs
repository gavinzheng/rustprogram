extern crate regex;
use regex::Regex;
fn main() {
  //"r" stands for "raw" strings, you probably
  // need them because rustc checks escape sequences,
  // although you can always use "\\" withour "r"
  let num_regex = Regex::new(r"\d+").unwrap();
  // is_match checks if string matches the pattern
  assert!(num_regex.is_match("some string with number 1"));
  let example_string = "some 123 numbers";
  // Regex::find searches for pattern and returns Option<(usize,usize)>,
  // which is either indexes of first and last bytes of match
  // or "None" if nothing matched
  match num_regex.find(example_string) {
    // Get the match slice from string, prints "123"
    Some(x) => println!("{}", &example_string[x.0 .. x.1]),
    None => unreachable!()
  }
}