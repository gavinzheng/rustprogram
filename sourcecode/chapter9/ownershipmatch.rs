#[derive(Debug)] 
enum GradeLevel{ 
  Freshman,
  Sophomore, 
  Junior,
  Senior,
} 
#[derive(Debug)] 
struct Student{ 
  Grade: GradeLevel
} 
fn main() { 
  let student= Student{ Grade: GradeLevel::Junior }; 
  match student.Grade { 
    GradeLevel::Freshman=> println!("The student is a freshman"), 
    a => println!("The student is a {:?}", a) 
  } 
  println!("{:?}", student);  // Error
} 