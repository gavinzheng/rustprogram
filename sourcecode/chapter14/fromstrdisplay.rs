use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point ({}, {})", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tuple: Vec<&str> = s.split(",").collect();
        let x = tuple[0].parse::<i32>()?;
        let y = tuple[1].parse::<i32>()?;
        Ok(Point {x:x, y:y})
    }
}

fn main() {
    let p1 = Point {x:100, y:200};
    let p2: Point = "300,400".parse().unwrap();
    println!("P1: {}\nP2: {}", p1, p2);
}