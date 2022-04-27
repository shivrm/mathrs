use crate::mathrs::lexer::Ops;
use crate::mathrs::parser::AstNode;

/// Recursively the value of an AST
pub fn interpret(node: AstNode) -> f64 {
    match node {
        AstNode::Number(n) => n as f64,
        
        AstNode::BinOp {left, op, right} => {
            let left = interpret(*left);
            let right = interpret(*right);

            match op {
                Ops::Add => left + right,
                Ops::Sub => left - right,
                Ops::Mul => left * right,
                Ops::Div => left / right,
                Ops::Pow => left.powf(right)
            }
        }
        
        AstNode::UnOp {operand, op} => {{
            let operand = interpret(*operand);
            
            match op {
                Ops::Add => operand,
                Ops::Sub => -operand,
                _ => panic!("Unexpected unary operator {op:?}")
            }
        }}
    }
}