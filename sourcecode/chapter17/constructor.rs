struct Student {
    name: &'static str,
    score: u32,
    level: u32
}

impl Student {
    fn new(s: &'static str, mut score: u32, d: u32) -> Self {
        if score > 100 { score = 100; }
        Student { name: s, score: score, level: d }
    }

    pub fn default() -> Self {
        Student::new("Milly", 100, 10)
    }
    
    pub fn give_score(h: u32) -> Self {
        Student::new("Gavin", h, 5)
    }
}   

fn main() {
    let al1 = Student{ name: "Alice", score: 100, level: 5 };
    let al2 = Student::new("Bob", 150, 15);
    println!("Student 1 is a {} and at {} level", al1.name, al1.level);
    let al3 = Student::default();
    println!("Student 3 is a {} and at {} level", al3.name, al3.level);
    let al4 = Student::give_score(75);
    println!("Student 4 is a {} and at {} level", al4.name, al4.level);
}


