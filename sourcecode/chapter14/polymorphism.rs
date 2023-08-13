
trait Shape {
  fn area(&self) ;
}

struct Circle {
  radius: f64,
}

struct Rectangle {
  width: f64,
  height: f64,
}

impl Shape for Circle {
  fn area(&self) {
      println!("Drawing a circle with radius {}", self.radius);
  }
}

impl Shape for Rectangle {
  fn area(&self) {
      println!("Drawing a rectangle with width {} and height {}", self.width, self.height);
  }
}


fn draw_object(d: &dyn Shape) {
  d.area();
}

fn main() {
  // Implement the Shape trait for Circle, Rectangle, and other structs...

  let mut shapes: Vec<Box<dyn Shape>> = Vec::new();
  shapes.push(Box::new(Circle { radius: 5.0 }));
  shapes.push(Box::new(Rectangle { width: 4.0, height: 6.0 }));

  for shape in shapes {
    //println!("Shape area: ", shape.area());
    shape.area();
  }
}