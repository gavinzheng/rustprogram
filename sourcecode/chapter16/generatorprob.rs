#![feature(generators, generator_trait)]
fn main(){
    let _gen = || {
        let s = "Generator testing".to_string();
        let ref_s = &s;
        yield ref_s.len();
        println!("{}", ref_s);
    };
}