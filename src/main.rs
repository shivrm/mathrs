use std::io;
use std::io::prelude::*;
mod mathrs;

fn main() {
    print!("Enter an expression: ");
    io::stdout().flush().expect("Could not flush buffer");

    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    let result = mathrs::eval(&mut text).unwrap();
    println!("Value: {result}");
}