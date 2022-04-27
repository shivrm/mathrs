mod lexer;
mod parser;
mod interpreter;

pub use lexer::Lexer;
pub use parser::Parser;
pub use interpreter::interpret;

pub struct Error {
    title: String,
    desc: String
}

/// Provides a single function that does lexing, parsing, and interpretation
pub fn eval(text: &mut str) -> Result<f64, Error> {
    let mut p = Parser::new(text);
    let ast = p.parse_expr()?;
    Ok(interpret(ast))
}