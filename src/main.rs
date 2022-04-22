use std::{io, fmt};
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum TokenGroups {
    Whitespace,
    LParen,
    RParen,
    Number,
    Operator,
    UnaryOp,
    Null // Used as initial `last_token_value` in lexer
}

// Token groups that should not group when they occur consecutively
// For example `((` should be interpreted as two seperate brackets
const NON_GROUPING_TOKENS: &'static [TokenGroups] = &[
    TokenGroups::LParen,
    TokenGroups::RParen,
    TokenGroups::Operator
];

#[derive(Debug)]
struct Token {
    group: TokenGroups,
    value: String,
    
    line: usize,
    col_start: usize,
    col_end: usize,
}

struct MathError {
    title: String,
    description: String,
    token: Token
}

impl fmt::Debug for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{} at line {}, cols {}-{}\n{}", self.title, self.token.line, self.token.col_start, self.token.col_end, self.description)
    }
}

fn lex(source: &str) -> Result<Vec<Token>, MathError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut last_word = String::new();

    let mut last_token_group = TokenGroups::Null;
    let mut last_token_start: usize = 0;

    let mut line = 1;

    for (i, c) in source.chars().enumerate() {
        // Interpret character as correct group
        let group = match c {
            '\n' => {
                line += 1;
                TokenGroups::Whitespace
            }
            ' ' | '\t' | '\r' => TokenGroups::Whitespace,
            '(' => TokenGroups::LParen,
            ')' => TokenGroups::RParen,
            '0'..='9' => TokenGroups::Number,
            '+' | '*' | '-' | '/' | '^' => TokenGroups::Operator,

            _ => return Err(MathError {
                title: "Could not lex token".to_owned(),
                description: "The lexer could not lex the token".to_owned(),
                token: Token {
                    group: TokenGroups::Null,
                    value: String::new(),
                    line,
                    col_start: last_token_start,
                    col_end: i
                }
            })
        };

        if group == last_token_group {
            // If token is non-grouping, then add it as a seperate token
            if let Some(_) = NON_GROUPING_TOKENS.iter().position(|&t| t == last_token_group) {
                let token = Token{
                    group: last_token_group,
                    value: last_word,
                    line,
                    col_start: last_token_start,
                    col_end: i
                };
                last_token_group = group;
                last_token_start = i;

                tokens.push(token);
                last_word = String::new();
            }
            last_word.push(c);
        
        } else {
            let token = Token{
                group: last_token_group,
                value: last_word,
                line,
                col_start: last_token_start,
                col_end: i
            };

            last_token_group = group;
            last_token_start = i;

            if token.group != TokenGroups::Null && token.group != TokenGroups::Whitespace {
                tokens.push(token);
            }
            last_word = String::from(c);
        }
    };
    Ok(tokens)
}

fn parse(tokens: Vec<Token>) -> Result<Vec<Token>, MathError> {
    // https://en.wikipedia.org/wiki/Shunting_yard_algorithm

    // Operator precedences for calculating order of operations
    let precedences: HashMap<String, u8> = HashMap::from([
        ("^".to_owned(), 3),
        ("/".to_owned(), 2), ("*".to_owned(), 2),
        ("+".to_owned(), 1), ("-".to_owned(), 1),
    ]);

    let mut op_stack: Vec<Token> = Vec::new();
    let mut result: Vec<Token> = Vec::new();
    let mut last_token_group = TokenGroups::Null;
        
    for token in tokens {
        let token_group = token.group;

        match token.group {
            TokenGroups::Number => {
                // Multiply if numbers occur consecutively
                if last_token_group == TokenGroups::Number {
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
                if last_token_group == TokenGroups::Operator || last_token_group == TokenGroups::Null {
                    op_stack.push(Token {
                        group: TokenGroups::UnaryOp,
                        ..token
                    });
                    continue;
                }

                let current_precedence = precedences[&token.value];                
                while !op_stack.is_empty() {
                    let operator = op_stack.pop().unwrap();
                    
                    // Left brace means that further operators are in a different scope
                    if operator.group == TokenGroups::LParen {
                        op_stack.push(operator);
                        break;
                    }
                    
                    let top_precedence = precedences[&operator.value];
                    if top_precedence > current_precedence || 
                      (top_precedence == current_precedence && &token.value != "^") {
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
            _ => return Err(MathError {
                title: "Unparsable Token".to_owned(),
                description: "Token could not be parsed".to_owned(),
                token
            })
        };

        last_token_group = token_group;
    };

    // If there are any operators left in stack, move them to result
    while !op_stack.is_empty() {
        result.push(op_stack.pop().unwrap());
    }

    Ok(result)
}

fn eval(tokens: Vec<Token>) -> Result<f64, MathError> {
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
                    "^" => left.powf(right),
                    _ => return Err(MathError {
                        title: "Unknown Operator".to_owned(),
                        description: "No handler exists for the operator".to_owned(),
                        token
                    })
                };
                result.push(value);
            }
            TokenGroups::UnaryOp => {
                let operand = result.pop().unwrap();

                let value = match &token.value[..] {
                    "+" => operand,
                    "-" => -operand,
                    _ => return Err(MathError {
                        title: "Unknown Operator".to_owned(),
                        description: "No handler exists for unary operator".to_owned(),
                        token
                    })
                };
                result.push(value);
            }
            _ => return Err(MathError {
                title: "Unevaluatable Token".to_owned(),
                description: "No handler exists for the token type".to_owned(),
                token
            })
        }
    };

    return Ok(result[0])
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

    let tokens = lex(&input).unwrap();
    let shunted = parse(tokens).unwrap();
    let value = eval(shunted).unwrap();

    println!("Value of expression: {}", value);
}