fn main(){ 
  struct Student { name: String, score: i32 } 
  let mut classstudents = Vec::new(); 
  classstudents.push(Student { name: "alice".to_string(), score: 85 }); 
  classstudents.push(Student { name: "bob".to_string(), score: 63 }); 
  classstudents.push(Student { name: "milly".to_string(), score: 56 }); 
  for stu in &classstudents{ 
  println!("{}, score {}", stu.name, stu.score); 
  } 
  }