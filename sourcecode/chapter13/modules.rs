use mod1::func2;
use mod1::func2 as mf2;
// use mod1::{func2, func3};
// pub use mod1::{func2, func3}; // visible in the super level
// use mod1::*;
use mod1::submod1::subfunc1 as sf1;

mod mod1 {
    // all of the module's code items go in here
    fn func1() { // private function
    	println!("Am I visible?");
    }

    pub fn func2() {
    	println!("You called func2 in mod1!");
    }

     pub fn func3() {
        println!("You called func3 in mod1!");
    }

    pub mod submod1 {
    	pub fn subfunc1() {
    		println!("You called subfunc1 in submod1!");
    	}
    }
}

fn main() {
	// game1::func1(); // error: function `func1` is private
    mod1::func2(); // works without the use import
    self::mod1::func2();
    ::mod1::func2();
    // super::game1::func2(); // unresolved name

	// calling a nested module:
    mod1::submod1::subfunc1();

	// importing a function or module with use:
    func2();
    mf2();
    sf1();
}
