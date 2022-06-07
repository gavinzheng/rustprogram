use std::thread;

fn main() {

/*
    let p1= Philosopher::new("Baruch Spinoza");
    let p1= Philosopher::new("Gilles Deleuze");
    let p1= Philosopher::new("Karl Marx");
    let p1= Philosopher::new("Friedrich Nietzsche");
    let p1= Philosopher::new("Michel Foucault");
  */
  
    let philosophers= vec![
        Philosopher::new("Baruch Spinoza"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Friedrich Nietzsche"),
        Philosopher::new("Michel Foucault"),
    ];
    
    let handles:Vec<_> = philosophers.into_iter().map(|p|{
        thread::spawn(move || {
            p.eat();
        })
    }).collect();
    
    for h in handles{
        h.join().unwrap();
    }
}



struct Philosopher{
    name:String,
}

impl Philosopher{
    fn new(name:&str)->Philosopher{
        Philosopher{
            name:name.to_string()
        }
    }
    
    fn eat(&self){
    
        println!("{} is eating.",self.name);
        
        thread::sleep_ms(1000);
    
        println!("{} is done eating",self.name);
    
    }

}
