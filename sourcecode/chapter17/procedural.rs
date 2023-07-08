use std::str::Split;
use std::num::ParseFloatError;

#[derive(Debug)]
struct Student{
    name: String,
    gpa : f32,
}
fn main() {
  let students :Vec<&str>= vec![
    "Alice 3.1",
    "Bob 2.3",
    "Cathy 3.5",
    "Milly 4.0",
    "Weide 3.9"
  ];

  let mut good_students:Vec<_>=vec![];
  for s in students{

    let mut s: Split<char> = s.split(' ');
    let name: Option<&str> = s.next();
    let gpa : Option<&str>= s.next();

    if name.is_some() && gpa.is_some(){
      let name : String = name.unwrap().to_owned();
      let gpa: String =gpa.unwrap().to_owned();
      let gpa: Result<f32,ParseFloatError> = gpa.parse::<f32>();
      if gpa.is_ok(){
        let gpa:f32=gpa.unwrap();
        if gpa >= 3.5 {
          good_students.push(Student{name,gpa});
        }
      }
    }
  }
  for s in good_students {
    println!("{:?}", s);
  }     

}