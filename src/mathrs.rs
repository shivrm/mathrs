mod lexer;
mod parser;
mod interpreter;

pub use lexer::Lexer;
pub use parser::Parser;
pub use interpreter::interpret;

pub fn eval(text: &mut str) -> Option<i32> {
    let mut p = Parser::new(text);
    let ast = p.parse_expr()?;
    Some(interpret(ast))
}