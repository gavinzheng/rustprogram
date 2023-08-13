pub trait Hei {
  fn hei(&self);
  fn weird(&self);
  fn need_sized(self) -> Self where Self: Sized;
  fn new()->Self where Self: Sized;
  fn foo()->Self where Self: Sized;
}

impl Hei for String {
  fn hei(&self) {
    println!("hei {}", self);
  }

  fn weird(&self) {
    println!("you called wierd {}", self);
  }

  fn need_sized(self) -> Self where Self: Sized{
    self
  }
  fn new() ->Self{
    println!("in foo()");
    "in foo()".to_owned()
  }
  fn foo() ->Self{
    println!("in foo()");
    "in foo()".to_owned()
  }
}

fn main() { 
   // let x = "hei".to_string(); 
  // x.need_sized(); 
   let message = String::from("hello!");
  message.need_sized().to_string();
  //let p = &message as &dyn Hei; 
  // p.need_sized();  // Error
  // String::foo(); //OK
} 
