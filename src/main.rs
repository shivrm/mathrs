use std::io;
use std::io::prelude::*;
mod mathrs;

fn main() {
    print!(concat!(
        "Welcome to mathrs, a math expression parser!\n",
        "Please enter your math expression.\n",
        "You can use multiple line to enter your expression.\n",
        "Entering a blank line will start evaluation.\n\n"
    ));
    io::stdout().flush().expect("Could not flush buffer");

    let mut text = String::new();
    
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if line.trim().is_empty() {
            break;
        }
        text = text + &line;
    }

    match mathrs::eval(&mut text) {
        Ok(n) => println!("Value {n}"),
        Err(e) => println!("Error: {e}")
    }
}