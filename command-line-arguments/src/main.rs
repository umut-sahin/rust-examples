use std::env;

fn main() {
    for (i, argument) in env::args().enumerate() {
        println!("{}: {}", i, argument);
    }
}
