use crate::mathrs::lexer::*;
use crate::mathrs::Error;

/// Compares if the variants of two enums are the same (ignoring values)
fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

/// Represents an operator during shunting.
/// The bool indicates if the operator `is_unary`
type ShuntOp = (Ops, bool);



/// Enum used to define different kinds of AST nodes
#[derive(Debug)]
pub enum AstNode {
    BinOp {
        left: Box<AstNode>,
        op: Ops,
        right: Box<AstNode>,
    },
    UnOp {
        op: Ops,
        operand: Box<AstNode>,
    },
    Number(i32),
    Float(f64)
}

/// Used to parse the code
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    line: usize,
    col: usize,
    depth: isize
}

impl<'a> Parser<'a> {
    /// Creates a new parser
    pub fn new(text: &'a str) -> Result<Self, Error> {
        let mut lexer = Lexer::new(text);
        let (token, line, col) = lexer.next_token()?;
        Ok(Parser {
            current_token: token,
            line,
            col,
            lexer,
            depth: 0
        })
    }
    
    /// Tells the parser to advance to the next token.
    /// Should be used instead of `expect` when it is guaranteed that
    /// token will be of the correct type.
    #[inline]
    fn advance(&mut self) -> Result<(), Error> {
        let (token, line, col) = self.lexer.next_token()?;

        match token {
            Token::OpenParen => self.depth += 1,
            Token::CloseParen => self.depth -= 1,
            Token::EOF => {
                if self.depth > 0 {
                    return Err(Error {
                        title: "Unclosed parenthesis".to_owned(),
                        desc: "EOF occurred while some parentheses were unclosed".to_owned(),
                        line: self.line, col: self.col
                    })
                }
            }
            _ => (())
        }

        if self.depth < 0 {
            return Err(Error {
                title: "Mismatched closing parenthesis".to_owned(),
                desc: "Closing parenthesis without matching opening was detected".to_owned(),
                line: self.line, col: self.col
            })
        }

        self.current_token = token;
        self.line = line;
        self.col = col;
        Ok(())
    }
    
    /// Tells the parser to expect a token of a type
    fn expect(&mut self, token_type: Token) -> Result<(), Error> {
        if !variant_eq(&self.current_token, &token_type) {
            return Err(Error {
                title: format!("Expected {token_type:?}"),
                desc: format!("Current token was {:?}", self.current_token),
                line: self.line, col: self.col
            })
        }
        self.advance()
    }
    
    /// Parses an operand
    ///
    /// `operand ::= (operator)operand | operand | '(' expr ')'`
    fn parse_operand(&mut self) -> Result<AstNode, Error> {
        match self.current_token {
            Token::Number(n) => {
                self.advance()?;

                if let Token::Period = self.current_token {
                    self.advance()?;
                    if let Token::Number(d) = self.current_token {
                        self.advance()?;
                        let value: f64 = format!("{n}.{d}").parse().unwrap();
                        Ok(AstNode::Float(value))
                    } else {
                        Err(Error {
                            title: "Decimal point without decimal digits".to_owned(),
                            desc: "Parser found a decimal point with no following digits".to_owned(),
                            line: self.line, col: self.col
                        })
                    }
                }
                else {
                    Ok(AstNode::Number(n))
                }
            }
            
            Token::OpenParen => {
                self.advance()?;
                let node = self.parse_expr();
                self.expect(Token::CloseParen)?;
                
                node
            }
            
            Token::EOF => Err(Error {
                title: "Unexpected EOF while reading operand".to_owned(),
                desc: "I don't really have a description for this".to_owned(),
                line: self.line, col: self.col
            }),
            
            _ => Err(Error {
                title: "Unexpected token in operand".to_owned(),
                desc: "Operand should start with number, unary operator, or parenthesis".to_owned(),
                line: self.line, col: self.col
            }),
        }
    }
    
    /// Returns the precedence of an operator
    fn precedence(&mut self, (op, unary): ShuntOp) -> Result<usize, Error> {
        let p = if unary {
            match op {
                Ops::Add => 3,
                Ops::Sub => 3,
                _ => return Err(Error {
                    title: "Invalid unary operator".to_owned(),
                    desc: "Only + and - can be used as unary operators".to_owned(),
                    line: self.line, col: self.col
                })
            }
        } else {
            match op {
                Ops::Pow => 4,
                Ops::Div => 2,
                Ops::Mul => 2,
                Ops::Add => 1,
                Ops::Sub => 1,
            }
        };
        Ok(p)
    }
    
    /// Pushes an operator as an AST node onto a stack of nodes
    fn push_op(&mut self, (op, unary): ShuntOp, mut nodes: Vec<AstNode>) -> Result<Vec<AstNode>, Error> {
        if unary {
            // Unary operators have only a single operand
            let operand = match nodes.pop() {
                Some(n) => n,
                None => return Err(Error {
                    title: "Missing argument for operator".to_owned(),
                    desc: "Node stack was empty when looking for argument".to_owned(),
                    line: self.line, col: self.col
                })
            };
            nodes.push(AstNode::UnOp {
                operand: Box::new(operand),
                op
            })
        } else {
            // Right comes before left because it is stack based
            let right = match nodes.pop() {
                Some(n) => n,
                None => return Err(Error {
                    title: "Missing argument for operator".to_owned(),
                    desc: "Node stack was empty when looking for argument".to_owned(),
                    line: self.line, col: self.col
                })
            };
            let left = match nodes.pop() {
                Some(n) => n,
                None => return Err(Error {
                    title: "Missing argument for operator".to_owned(),
                    desc: "Node stack was empty when looking for argument".to_owned(),
                    line: self.line, col: self.col
                })
            };
    
            nodes.push(AstNode::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        }
        Ok(nodes)
    }
    
    /// Shunts an operator
    fn shunt_op(
        &mut self,
        op: ShuntOp,
        mut nodes: Vec<AstNode>,
        mut ops: Vec<ShuntOp>,
    ) -> Result<(Vec<AstNode>, Vec<ShuntOp>), Error> {

        // Unary operator check
        if op.1 && op.0 != Ops::Add && op.0 != Ops::Sub {
            return Err(Error {
                title: "Invalid unary operator".to_owned(),
                desc: "Only + and - can be used as unary operators".to_owned(),
                line: self.line, col: self.col
            })
        }

        // While operator stack is not empty check if top operator has higher precedence
        // and apply that first... (https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
        
        while !ops.is_empty() {
            let top_op = ops.pop().unwrap();
            if self.precedence(top_op)? > self.precedence(op)?
                || (self.precedence(top_op)? == self.precedence(op)? && top_op.0 != Ops::Pow)
            {
                nodes = self.push_op(top_op, nodes)?;
            } else {
                ops.push(top_op);
                break;
            }
        }
        ops.push(op);
        Ok((nodes, ops))
    }

    /// Parses an expression
    ///
    /// `expr ::= operand ((operator)operand)*`
    // Uses shunting-yard algorithm, modified for ASTs
    pub fn parse_expr(&mut self) -> Result<AstNode, Error> {
        let mut nodes: Vec<AstNode> = Vec::new(); // Used to keep track of AST nodes
        let mut ops: Vec<ShuntOp> = Vec::new(); // Used as operator stack

        // Parse initial unary operators
        while let Token::Operator(op) = self.current_token {
            self.advance()?;
            (nodes, ops) = self.shunt_op((op, true), nodes, ops)?;
        }

        nodes.push(self.parse_operand()?);

        // Parse subsequent operator-operand pairs
        while let Token::Operator(op) = self.current_token {
            self.advance()?;
            (nodes, ops) = self.shunt_op((op, false), nodes, ops)?;            

            // Every operator should be followed by an operand

            // Parse any unary operators before operand
            while let Token::Operator(op) = self.current_token {
                self.advance()?;
                (nodes, ops) = self.shunt_op((op, true), nodes, ops)?;
            }

            let operand = self.parse_operand()?;
            nodes.push(operand);
        }

        // If going out of scope, or at end of text, finish up the shunting
        // Move remaining operators from operator stack to node stack
        if let Token::EOF | Token::CloseParen = self.current_token {
            while !ops.is_empty() {
                let op = ops.pop().unwrap();
                nodes = self.push_op(op, nodes)?;
            }
            return match nodes.pop() {
                Some(n) => Ok(n),
                None => Err(Error {
                    title: "No node to return".to_owned(),
                    desc: "Node stack was empty when looking for a node to return".to_owned(),
                    line: self.line, col: self.col
                })
            }
        } else {
            Err(Error {
                title: "Unexpected token after expression".to_owned(),
                desc: format!("{:?} is not allowed after an expression", self.current_token),
                line: self.line, col: self.col
            })
        }
    }
}
