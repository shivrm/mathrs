use std::io;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum TokenGroups {
    Whitespace,
    LParen,
    RParen,
    Number,
    Operator,
    Null // Used as initial `last_token_value` in lexer
}

// Token types that should not group when they occur consecutively
// For example `((` should be interpreted as two seperate brackets
const NON_GROUPING_TYPES: &'static [TokenGroups] = &[
    TokenGroups::LParen,
    TokenGroups::RParen,
    TokenGroups::Operator
];

#[derive(Debug)]
struct Token {
    group: TokenGroups,
    value: String,
    
    row: usize,
    col_start: usize,
    col_end: usize,
}

fn error(token: &Token, message: &str) {
    eprintln!("On line {}, columns {}-{}", token.row, token.col_start, token.col_end);
    eprintln!("{}", message);
}

fn lex(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut last_word = String::new();
    let mut last_token_type = TokenGroups::Null;
    let mut last_token_start: usize = 0;
    let mut row = 1;

    for (i, c) in source.chars().enumerate() {
        // Interpret character as correct type
        let group = match c {
            ' ' | '\n' | '\t' | '\r' => {
                if c == '\n' {
                    row += 1;
                }
                TokenGroups::Whitespace
            },
            '(' => TokenGroups::LParen,
            ')' => TokenGroups::RParen,
            '0'..='9' => TokenGroups::Number,
            '+' | '*' | '-' | '/' => TokenGroups::Operator,
            _ => {
                eprintln!("On line {}, column {}", row, last_token_start);
                panic!("Token could not be parsed - {}", c);
            }
        };

        if group == last_token_type {
            // If token is non-grouping, then add it as a seperate token
            if let Some(_) = NON_GROUPING_TYPES.iter().position(|&t| t == last_token_type) {
                let token = Token{
                    group: last_token_type,
                    value: last_word,
                    row: row,
                    col_start: last_token_start,
                    col_end: i
                };
                last_token_type = group;
                last_token_start = i;

                tokens.push(token);
                last_word = String::new();
            }
        
            last_word.push(c);
        } else {
            let token = Token{
                group: last_token_type,
                value: last_word,
                row: row,
                col_start: last_token_start,
                col_end: i
            };

            last_token_type = group;
            last_token_start = i;

            if token.group != TokenGroups::Null && token.group != TokenGroups::Whitespace {
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
    let precedences: HashMap<String, u8> = HashMap::from([
        ("/".to_owned(), 2), ("*".to_owned(), 2),
        ("+".to_owned(), 1), ("-".to_owned(), 1),
    ]);

    let mut op_stack: Vec<Token> = Vec::new();
    let mut result: Vec<Token> = Vec::new();
    let mut last_token_type = TokenGroups::Null;
        
    for token in tokens {
        let token_type = token.group;

        match token.group {
            TokenGroups::Number => {
                // Multiply if numbers occur consecutively
                if last_token_type == TokenGroups::Number {
                    op_stack.push(Token {
                        group: TokenGroups::Operator,
                        value: "*".to_owned(),
                        ..token
                    });
                }
                result.push(token);
            },
            TokenGroups::LParen => op_stack.push(token),
            TokenGroups::Operator => {
                let current_precedence = precedences[&token.value];                
                while !op_stack.is_empty() {
                    let operator = op_stack.pop().unwrap();
                    
                    // Left brace means that further operators are in a different scope
                    if operator.group == TokenGroups::LParen {
                        op_stack.push(operator);
                        break;
                    }
                    
                    let top_precedence = precedences[&operator.value];
                    if top_precedence >= current_precedence {
                        result.push(operator);
                    }
                };
                op_stack.push(token);
            },
            TokenGroups::RParen => {
                loop {
                    if op_stack.is_empty() {
                        // There should've been a matching left paranthesis
                        panic!("Mismatched Parentheses");
                    }

                    let top_operator = op_stack.pop().unwrap();

                    if top_operator.group == TokenGroups::LParen {
                        break;
                    } else {
                        result.push(top_operator);
                    }
                }
            },
            _ => {
                error(&token, &format!("Token {} could not be handled", token.value));
            }
        };

        last_token_type = token_type;
    };

    // If there are any operators left in stack, move them to result
    while !op_stack.is_empty() {
        result.push(op_stack.pop().unwrap());
    }

    return result
}

fn eval(tokens: &Vec<Token>) -> f64 {
    let mut result: Vec<f64> = Vec::new();

    for token in tokens {
        match token.group {
            TokenGroups::Number => {
                let as_float: f64 = token.value.parse().unwrap();
                result.push(as_float);
            }
            TokenGroups::Operator => {
                // Shunting-yard algorithm, being stack based, puts the
                // later operand on top. So right comes on top of left
                let right = result.pop().unwrap();
                let left = result.pop().unwrap();

                let value = match &token.value[..] {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    _ => panic!("Unhandled operator")
                };
                result.push(value);
            }
            _ => error(&token, &format!("Token {} could not be evaluated", token.value))
        }
    };

    return result[0]
}

fn main() {
    let mut input = String::new();
    let mut line = String::new();

    loop {
        io::stdin()
        .read_line(&mut line)
        .expect("Failed to read input");

        if line.trim().len() == 0 {break;}
        input = input + &line;
        line.clear();
    }

    let tokens = lex(&input);
    let shunted = shunt(tokens);
    let value = eval(&shunted);

    println!("Value of expression: {}", value);
}