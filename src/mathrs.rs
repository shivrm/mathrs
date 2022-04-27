mod lexer;
mod parser;
mod interpreter;

pub use lexer::Lexer;
pub use parser::Parser;
pub use interpreter::interpret;

/// Provides a single function that does lexing, parsing, and interpretation
pub fn eval(text: &mut str) -> Option<f64> {
    let mut p = Parser::new(text);
    let ast = p.parse_expr()?;
    Some(interpret(ast))
}