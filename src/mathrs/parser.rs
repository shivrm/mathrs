use crate::mathrs::lexer::*;

/// Compares if the variants of two enums are the same (ignoring values)
fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

/// Returns the precedence of an operator
fn precedence(op: Ops) -> usize {
    match op {
        Ops::Pow => 3,
        Ops::Div => 2,
        Ops::Mul => 2,
        Ops::Add => 1,
        Ops::Sub => 1
    }
}

/// Enum used to define different kinds of AST nodes
#[derive(Debug)]
pub enum AstNode {
    BinOp {
        left: Box<AstNode>,
        op: Ops,
        right: Box<AstNode>
    },
    UnOp {
        op: Ops,
        operand: Box<AstNode>
    },
    Number(i32)
}

/// Used to parse the code
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token
}

impl<'a> Parser<'a> {
    /// Creates a new parser
    pub fn new(text: &'a str) -> Self {
        let mut lexer = Lexer::new(text);
        Parser {
            current_token: lexer.next_token(),
            lexer
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
    fn parse_operand(&mut self) -> Option<AstNode> {
        match self.current_token {

            Token::Operator(op) => {
                self.advance();
                let operand = self.parse_operand()?;

                Some(AstNode::UnOp {
                    op,
                    operand: Box::new(operand)
                })
            }

            Token::Number(n) => {
                self.advance();
                Some(AstNode::Number(n))
            }

            Token::OpenParen => {
                self.advance();
                let node = self.parse_expr();
                self.expect(Token::CloseParen);

                node
            }
            _ => panic!("Unexpected token")
        }
    }

    /// Parses an expression
    /// 
    /// `expr ::= operand ((operator)operand)*`
    // Uses shunting-yard algorithm, modified for ASTs
    // TODO: Move unary operator parsing here, 
    pub fn parse_expr(&mut self) -> Option<AstNode> {
        // Used to keep track of AST nodes
        let mut nodes: Vec<AstNode> = Vec::new();
        nodes.push(self.parse_operand()?);
        
        // Used as operator stack
        let mut ops: Vec<Ops> = Vec::new();

        // Parse subsequent operator-operand pairs
        while let Token::Operator(op) = self.current_token {
            self.advance();
            
            // Handle operator precedence
            while !ops.is_empty() {
                let top_op = ops.pop()?;
                if precedence(top_op) > precedence(op) || (
                    precedence(top_op) == precedence(op) && top_op != Ops::Pow
                ) {
                    
                    // Operands for the operator. right comes first as it is stack-based
                    let right = nodes.pop()?;
                    let left = nodes.pop()?;

                    // Add operator to AST node stack
                    nodes.push(AstNode::BinOp {
                        left: Box::new(left),
                        op: top_op,
                        right: Box::new(right)
                    })
                } else {
                    ops.push(top_op);
                    break;
                }
            }
            ops.push(op);

            // Every operator should be followed by an operand
            let operand = self.parse_operand()?;
            nodes.push(operand);
        }

        // If going out of scope, or at end of text, finish up the shunting
        // Move remaining operators from operator stack to node stack
        if let Token::EOF | Token::CloseParen = self.current_token {
            while !ops.is_empty() {
                let op = ops.pop()?;
                let right = nodes.pop()?;
                let left = nodes.pop()?;

                nodes.push(AstNode::BinOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right)
                })
            }
            return Some(nodes.pop()?);
        } else {
            panic!("Unexpected Token");
        }
    }
}
