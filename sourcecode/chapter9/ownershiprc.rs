use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Student {
  name: String,
  no: i32,
  score :f64,
}

fn main() {
  let first_student: Rc<RefCell<Student>> = Rc::new(RefCell::new(
                Student  {
                  name: "张三".to_string(),
                  no: 1000,
                  score: 80.,
                }
              ));


  println!("first_student: {:?}", first_student);

  // { 
  //   let mut base_2 = base.borrow_mut();
  //   base_2.radio_freq -= 12.34;
  //   println!("base_2: {:?}", base_2);
  // }

  // println!("base: {:?}", base);

  heap_procedure(first_student.clone());

  let mut teacher_2 = first_student.borrow_mut();
  teacher_2.score += 5.5;

  println!("first_student: {:?}", first_student);
  println!("teacher_2: {:?}", teacher_2);
}
fn heap_procedure(param: Rc<RefCell<Student>>){
  println!("heap_procedure: {:?}", param);
}