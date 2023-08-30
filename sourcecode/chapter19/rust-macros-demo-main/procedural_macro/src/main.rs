use macro_demo::MyTrait;
use macro_demo_derive::MyMacro;
use macro_demo_derive::trace;
use macro_demo_derive::print_and_replace;

#[derive(MyMacro)]
struct MyStruct;

#[trace]
fn foo() {}

#[trace(some_arg)]
fn bar() {}

#[trace(some_arg1, some_arg2)]
fn baz() {}

fn main() {
    MyStruct::hello();
    print_and_replace!(100);
    add(1,2);
}
