
fn main() {
    cc::Build::new()
        .file("count-string.c")
        .static_flag(true)
        .compile("countstring");
}
// fn main() {

// }
