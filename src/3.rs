use std::fs;
fn main() {
    let contents = fs::read_to_string("main.rs").unwrap();
    println!("{}", contents);
}
