use crate::mathrs::lexer::*;

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn precedence(op: Ops) -> usize {
    match op {
        Ops::Pow => 3,
        Ops::Div => 2,
        Ops::Mul => 2,
        Ops::Add => 1,
        Ops::Sub => 1
    }
}

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

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut lexer = Lexer::new(text);
        Parser {
            current_token: lexer.next_token(),
            lexer
        }
    }

    fn expect(&mut self, token_type: Token) {
        if !variant_eq(&self.current_token, &token_type) {
            panic!("Expected {token_type:?}");
        }
        self.current_token = self.lexer.next_token();
    }

    fn parse_operand(&mut self) -> Option<AstNode> {
        match self.current_token {

            Token::Operator(op) => {
                self.expect(Token::Operator(op));
                let operand = self.parse_operand()?;

                Some(AstNode::UnOp {
                    op,
                    operand: Box::new(operand)
                })
            }

            Token::Number(n) => {
                self.expect(Token::Number(n));
                Some(AstNode::Number(n))
            }

            Token::OpenParen => {
                self.expect(Token::OpenParen);
                let node = self.parse_expr();
                self.expect(Token::CloseParen);

                node
            }
            _ => panic!("Unexpected token")
        }
    }

    pub fn parse_expr(&mut self) -> Option<AstNode> {
        let mut nodes: Vec<AstNode> = Vec::new();
        nodes.push(self.parse_operand()?);
        let mut ops: Vec<Ops> = Vec::new();

        while let Token::Operator(op) = self.current_token {
            self.expect(Token::Operator(op));
            
            while !ops.is_empty() {
                let top_op = ops.pop()?;
                if precedence(top_op) > precedence(op) || (
                    precedence(top_op) == precedence(op) && top_op != Ops::Pow
                ) {
                    let right = nodes.pop()?;
                    let left = nodes.pop()?;

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

            let operand = self.parse_operand()?;
            nodes.push(operand);
        }

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
