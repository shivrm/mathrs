use std::io;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum TokenTypes {
    Whitespace,
    LBrace,
    RBrace,
    Number,
    Operator,
    Null // Used as initial `last_token_value` in lexer
}

// Token types that should not group when they occur consecutively
// For example `((` should be interpreted as two seperate brackets
const NON_GROUPING_TYPES: &'static [TokenTypes] = &[
    TokenTypes::LBrace,
    TokenTypes::RBrace,
    TokenTypes::Operator
];

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
        // Interpret character as correct type
        let t_type = match c {
            ' ' | '\n' | '\t' | '\r' => TokenTypes::Whitespace,
            '(' => TokenTypes::LBrace,
            ')' => TokenTypes::RBrace,
            '0'..='9' => TokenTypes::Number,
            '+' | '*' | '-' | '/' => TokenTypes::Operator,
            _ => panic!("Invalid token")
        };

        if t_type == last_token_type {
            // If token is non-grouping, then add it as a seperate token
            if let Some(_) = NON_GROUPING_TYPES.iter().position(|&t| t == last_token_type) {
                let token = Token{t_type: last_token_type, t_value: last_word};
                last_token_type = t_type;

                tokens.push(token);
                last_word = String::new();
            }
        
            last_word.push(c);
        } else {
            let token = Token{t_type: last_token_type, t_value: last_word};
            last_token_type = t_type;
            
            if token.t_type != TokenTypes::Null {
                tokens.push(token);
            }
            last_word = String::from(c);
        }
    };

    return tokens
}

fn shunt(tokens: Vec<Token>) -> Vec<Token> {
    // https://en.wikipedia.org/wiki/Shunting_yard_algorithm

    // Operator precedences for calculating order of operations
    let OP_PRECEDENCES: HashMap<String, u8> = HashMap::from([
        ("/".to_owned(), 2), ("*".to_owned(), 2),
        ("+".to_owned(), 1), ("-".to_owned(), 1),
    ]);

    let mut op_stack: Vec<Token> = Vec::new();
    let mut result: Vec<Token> = Vec::new();

    for token in tokens {
        match token.t_type {
            TokenTypes::Number => result.push(token),
            TokenTypes::LBrace => op_stack.push(token),
            TokenTypes::Operator => {
                let current_precedence = OP_PRECEDENCES[&token.t_value];                
                while !op_stack.is_empty() {
                    let operator = op_stack.pop().unwrap();
                    
                    // Left brace means that further operators are in a different scope
                    if operator.t_type == TokenTypes::LBrace {
                        op_stack.push(operator);
                        break;
                    }
                    
                    let top_precedence = OP_PRECEDENCES[&operator.t_value];
                    if top_precedence >= current_precedence {
                        result.push(operator);
                    }
                };
                op_stack.push(token);
            },
            TokenTypes::RBrace => {
                loop {
                    if op_stack.is_empty() {
                        // There should've been a matching left paranthesis
                        panic!("Mismatched Parentheses");
                    }

                    let top_operator = op_stack.pop().unwrap();

                    if top_operator.t_type == TokenTypes::LBrace {
                        break;
                    } else {
                        result.push(top_operator);
                    }
                }
            },
            TokenTypes::Whitespace => {},
            _ => panic!("Unhandled token type: {:?}", token.t_type)
        };
    };

    // If there are any operators left in stack, move them to result
    while !op_stack.is_empty() {
        result.push(op_stack.pop().unwrap());
    }

    return result
}

fn eval(tokens: &Vec<Token>) -> f64 {
    let result: Vec<f64> = Vec::new();

    for token in tokens {
        match token.t_type {
            _ => unimplemented!()
        }
    };

    return result[0]
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let tokens = lex(&input);
    let shunted = shunt(tokens);
    let value = eval(&shunted);

    println!("Value of expression: {}", value);
}