use std::io;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum TokenTypes {
    Whitespace,
    LBrace,
    RBrace,
    Number,
    Operator,
    Null // Used as initial `last_token_value` in lexer
}

#[derive(Debug)]
struct Token {
    t_type: TokenTypes,
    t_value: String
}

fn lex(source: &str) -> Vec<Token> {
    let mut last_word = String::new();
    let mut last_token_type = TokenTypes::Null;
    
    for c in source.chars() {
        let t_type = match c {
            ' ' | '\n' | '\t' | '\r' => TokenTypes::Whitespace,
            '(' => TokenTypes::LBrace,
            ')' => TokenTypes::RBrace,
            '0'..='9' => TokenTypes::Number,
            '+' | '*' | '-' | '/' => TokenTypes::Operator,
            _ => panic!("Invalid token")
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