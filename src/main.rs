use std::io;
use std::io::prelude::*;
mod mathrs;

fn main() {
    print!(concat!(
        "mathrs v1.0.0\n",
        "Type 'bugs' or 'copyright for more information.\n",
        "Type 'q' or 'quit' to quit\n"
    ));
    io::stdout().flush().expect("Could not flush buffer");
    
    loop {
        print!(">>> ");
        io::stdout().flush().expect("Could not flush buffer");
    
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
            
        match &line.trim()[..] {
            "bugs" => println!("Report bugs at https://github.com/shivrm/mathrs/issues"),
            "copyright" => println!("Copyright (c) 2022 shivrm"),
            "quit" | "q" => {
                println!("Quitting");
                break;
            }
            _ => {
                match mathrs::eval(&mut line) {
                    Ok(n) => println!("\x1b[32m{n}\x1b[0m\n"),
                    Err(e) => println!("{e}")
                }
            }
        }

    }

}