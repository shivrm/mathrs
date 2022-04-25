use mathrs::lexer::*;

enum AstNode {
    BinOp {
        left: AstNode,
        op: Ops,
        right: AstNode
    },
    UnOp {
        op: Ops,
        operand: AstNode
    },
    Number(i32)
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token
}

