use std::fmt::Debug;
fn print_it<T: Debug + 'static>( input: &T) {
    println!( "'static value passed in is: {:?}", input );
}
fn main() {
    let i = 10;

    print_it(&i);
}