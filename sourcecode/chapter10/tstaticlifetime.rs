use std::fmt::Debug;

fn print_it<T: Debug + 'static>( input: T) {
    println!( "'static value passed in is: {:?}", input );
}

// fn print_it1( input: impl Debug + 'static ) {
//     println!( "'static value passed in is: {:?}", input );
// }



fn main() {
    let i = 5;

    print_it(&i);
    // print_it1(&i);
}

// use std::fmt::Debug;

// fn print_it<T: Debug + 'static>( input: &T) {
//     println!( "'static value passed in is: {:?}", input );
// }

// fn main() {
//     let i = 5;

//     print_it(&i);
// }
