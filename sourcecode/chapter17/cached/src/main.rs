#[macro_use] extern crate cached;
#[macro_use] extern crate lazy_static;

cached! {
    FIB;
    fn fib(n: u64) -> u64 = {
        if n==0 || n==1 { return n }
        println!("n: {}", n);
        fib(n-1) + fib(n-2)
    }
}
fn main()
{
    fib(30);
}