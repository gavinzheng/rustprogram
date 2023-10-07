#[macro_use]
extern crate helloworldderive;

// trait definitions have to be in "consumer" crate
trait HelloWorld {
    // This method will send a friendly greeting
    fn hello_world();
}

// thanks to the code in the custom_derive crate
// we can derive from HelloWorld in order to provide
// an automatic implementation for the HelloWorld trait
#[derive(HelloWorld)]
struct Alice;

#[derive(HelloWorld)]
struct Bob;

#[derive(HelloWorld)]
// We can use an optional attribute to change the message
#[hello_world_name = "Je aime Milly"]
struct Milly;

fn main() {
    Alice::hello_world();
    Bob::hello_world();
    Milly::hello_world();
}
