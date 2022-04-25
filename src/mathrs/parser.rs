use crate::mathrs::lexer::*;

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

enum AstNode {
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
        let left = self.parse_operand()?;
        
        match self.current_token {
            Token::Operator(op) => {
                self.expect(Token::Operator(Ops::Add));
                let right = self.parse_operand()?;

                Some(AstNode::BinOp {
                    left: Box::new(left),
                    op,
                    right: Box::new(right)
                })
            }

            Token::EOF => return Some(left),
            _ => panic!("Unexpected Token")
        }
    }
}
