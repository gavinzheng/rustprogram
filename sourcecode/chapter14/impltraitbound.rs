use std::fmt::Display; 
fn show_msg(val: impl Display) { 
println!("{}", val); 
} 
fn main() { 
show_msg("This is a test"); 
}