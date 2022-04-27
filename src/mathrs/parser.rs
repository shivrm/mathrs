use crate::mathrs::lexer::*;
use crate::mathrs::Error;

/// Compares if the variants of two enums are the same (ignoring values)
fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

/// Represents an operator during shunting.
/// The bool indicates if the operator `is_unary`
type ShuntOp = (Ops, bool);

/// Returns the precedence of an operator
fn precedence((op, unary): ShuntOp) -> usize {
    if unary {
        match op {
            Ops::Add => 3,
            Ops::Sub => 3,
            _ => panic!("Invalid unary operator")
        }
    } else {
        match op {
            Ops::Pow => 4,
            Ops::Div => 2,
            Ops::Mul => 2,
            Ops::Add => 1,
            Ops::Sub => 1,
        }
    }
}

/// Pushes an operator as an AST node onto a stack of nodes
fn push_op((op, unary): ShuntOp, mut nodes: Vec<AstNode>) -> Result<Vec<AstNode>, Error> {
    if unary {
        // Unary operators have only a single operand
        let operand = match nodes.pop() {
            Some(n) => n,
            None => return Err(Error {
                title: "Missing argument for operator".to_owned(),
                desc: "Node stack was empty when looking for argument".to_owned()
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
                desc: "Node stack was empty when looking for argument".to_owned()
            })
        };
        let left = match nodes.pop() {
            Some(n) => n,
            None => return Err(Error {
                title: "Missing argument for operator".to_owned(),
                desc: "Node stack was empty when looking for argument".to_owned()
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
    op: ShuntOp,
    mut nodes: Vec<AstNode>,
    mut ops: Vec<ShuntOp>,
) -> Result<(Vec<AstNode>, Vec<ShuntOp>), Error> {
    // While operator stack is not empty check if top operator has higher precedence
    // and apply that first... (https://en.wikipedia.org/wiki/Shunting-yard_algorithm)
    while !ops.is_empty() {
        let top_op = ops.pop().unwrap();
        if precedence(top_op) > precedence(op)
            || (precedence(top_op) == precedence(op) && top_op.0 != Ops::Pow)
        {
            nodes = push_op(top_op, nodes)?;
        } else {
            ops.push(top_op);
            break;
        }
    }
    ops.push(op);
    Ok((nodes, ops))
}

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
}

/// Used to parse the code
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    /// Creates a new parser
    pub fn new(text: &'a str) -> Self {
        let mut lexer = Lexer::new(text);
        Parser {
            current_token: lexer.next_token(),
            lexer,
        }
    }

    /// Tells the parser to advance to the next token.
    /// Should be used instead of `expect` when it is guaranteed that
    /// token will be of the correct type.
    #[inline]
    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    /// Tells the parser to expect a token of a type
    fn expect(&mut self, token_type: Token) {
        if !variant_eq(&self.current_token, &token_type) {
            panic!("Expected {token_type:?}");
        }
        self.current_token = self.lexer.next_token();
    }

    /// Parses an operand
    ///
    /// `operand ::= (operator)operand | operand | '(' expr ')'`
    fn parse_operand(&mut self) -> Result<AstNode, Error> {
        match self.current_token {
            Token::Number(n) => {
                self.advance();
                Ok(AstNode::Number(n))
            }

            Token::OpenParen => {
                self.advance();
                let node = self.parse_expr();
                self.expect(Token::CloseParen);

                node
            }
            _ => panic!("Unexpected token"),
        }
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
            self.advance();
            (nodes, ops) = shunt_op((op, true), nodes, ops)?;
        }

        nodes.push(self.parse_operand()?);

        // Parse subsequent operator-operand pairs
        while let Token::Operator(op) = self.current_token {
            self.advance();
            (nodes, ops) = shunt_op((op, false), nodes, ops)?;            

            // Every operator should be followed by an operand

            // Parse any unary operators before operand
            while let Token::Operator(op) = self.current_token {
                self.advance();
                (nodes, ops) = shunt_op((op, true), nodes, ops)?;
            }

            let operand = self.parse_operand()?;
            nodes.push(operand);
        }

        // If going out of scope, or at end of text, finish up the shunting
        // Move remaining operators from operator stack to node stack
        if let Token::EOF | Token::CloseParen = self.current_token {
            while !ops.is_empty() {
                let op = ops.pop().unwrap();
                nodes = push_op(op, nodes)?;
            }
            return match nodes.pop() {
                Some(n) => Ok(n),
                None => Err(Error {
                    title: "No node to return".to_owned(),
                    desc: "Node stack was empty when looking for a node to return".to_owned()
                })
            }
        } else {
            panic!("Unexpected Token");
        }
    }
}
