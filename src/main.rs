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
    let mut tokens: Vec<Token> = Vec::new();
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

        if t_type == last_token_type {
            last_word.push(c);
        } else {
            let token = Token{t_type: last_token_type, t_value: last_word};
            last_token_type = t_type;
            tokens.push(token);
            last_word = String::from(c);
        }
    };

    return tokens
}

fn shunt(tokens: &Vec<Token>) -> Vec<Token> {
    let mut op_stack: Vec<Token> = Vec::new();
    let mut result: Vec<Token> = Vec::new();

    for token in tokens {
        match token.t_type {
            _ => unimplemented!()
        };
    };

    return result
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