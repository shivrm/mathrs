use std::io;

struct Token {}

fn lex(source: &str) -> Vec<Token> {
    unimplemented!();
}

fn shunt(tokens: &Vec<Token>) -> Vec<Token> {
    unimplemented!();
}

fn eval(tokens: &Vec<Token>) -> f64 {
    unimplemented!();
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let tokens = lex(&input);
    let shunted = shunt(&tokens);
    let value = eval(&shunted);

    println!("Value of expression: {}", value);
}