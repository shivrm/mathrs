use crate::mathrs::lexer::Ops;
use crate::mathrs::parser::AstNode;

fn eval(node: AstNode) -> i32 {
    match node {
        AstNode::Number(n) => n,
        AstNode::BinOp {left, op, right} => {
            let left = eval(*left);
            let right = eval(*right);

            match op {
                Ops::Add => left + right,
                Ops::Sub => left - right,
                Ops::Mul => left * right,
                Ops::Div => left / right
            }
        }
        AstNode::UnOp {operand, op} => {{
            let operand = eval(*operand);
            
            match op {
                Ops::Add => operand,
                Ops::Sub => -operand,
                _ => panic!("Unexpected unary operator {op:?}")
            }
        }}
    }
}