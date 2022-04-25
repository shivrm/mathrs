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

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token
}

impl<'a> Parser<'a> {
    pub fn new(text: &'a str) -> Self {
        let mut lexer = Lexer::new(text);
        Parser {
            current_token: lexer.next_token().unwrap(),
            lexer
        }
    }

    fn expect(&mut self, token_type: Token) {
        if !variant_eq(&self.current_token, &token_type) {
            panic!("Expected {token_type:?}");
        }
        self.current_token = self.lexer.next_token().unwrap();
    }
}
