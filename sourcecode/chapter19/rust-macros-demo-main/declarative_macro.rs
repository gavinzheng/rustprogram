macro_rules! greetings {
    (struct $x: ident {}) => {
        println!("Hello from struct {}", stringify!($x));
    };
    ($x: expr) => {
        println!("Hello, {}", $x);
    };
}

macro_rules! add {
    ($a:expr)=>{
        $a
    };
    ($a:expr,$b:expr)=>{
        $a+$b
    };
    ($a:expr,$($b:tt)*)=>{
        $a+add!($($b)*)
    }
}

// macro_rules! foo {
//     ($x: expr) => {
//         a + $x
//     }
// }
fn main() {
    greetings!("Earthly"); // Prints "Hello, Earthly"
    greetings! {
        struct G {} // Prints "Hello from struct G"
    };
    println!("{}", add!(1));
    println!("{}", add!(1,2));
    println!("{}", add!(1,2,3));

    // let a = 42;
    // println!("{}", foo!(5));
}