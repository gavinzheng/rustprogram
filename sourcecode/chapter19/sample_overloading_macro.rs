//-- #########################
//-- Task: Overloading Macros
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 28 March 17
//-- #########################

// `test!` will compare `$left` and `$right`
// in different ways depending on how you invoke it:
macro_rules! test {
    // Arguments don't need to be separated by a comma.
    // Any template can be used!
    ($left:expr; and $right:expr) => (
        println!("{:?} and {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    );
    // ^ each arm must end with a semicolon.
    ($left:expr; or $right:expr) => (
        println!("{:?} or {:?} is {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    );
}

fn main() {
    test!(3i32 + 2 == 5i32; and 5i32 * 6 == 30i32);
    test!(true; or false);
}