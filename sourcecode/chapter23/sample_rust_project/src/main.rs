
#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

fn main(){
    let x = Some(1u8);
    match x {
        Some(y) => println!("{:?}", y),
        _ => ()
    }
}