#![feature(generators)]

fn main() {
    || {
        let x: u64 = 1;
        let ref_x: &u64 = &x;
        yield 0;
        yield *ref_x;
    };
    // Ok();
}