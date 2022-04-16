use std::io;

#[derive(Eq, PartialEq)]
enum TokenTypes {
    Whitespace,
    LBrace,
    RBrace,
    Number,
    Operator,
}

struct Token {}

fn lex(source: &str) -> Vec<Token> {
    let mut last_word = String::new();
    let mut last_token_type: TokenTypes;
    
    for char in source.chars() {
        let t_type = match char {
            ' ' | '\n' | '\t' => TokenTypes::Whitespace,
            '(' => TokenTypes::LBrace,
            ')' => TokenTypes::RBrace,
            '0'..='9' => TokenTypes::Number,
            '+' | '*' | '-' | '/' => TokenTypes::Operator,
        };  
    };
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