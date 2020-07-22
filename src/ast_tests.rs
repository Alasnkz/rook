#[cfg(test)]
use crate::ast::Parser;
#[cfg(test)]
use crate::lexer::Lexer;

#[test]
fn test_basic() {
    let mut p = Parser::new(Lexer::new("new a = 4;").lex());
    p.parse().expect("failed to parse");

    println!("{:?}", p.root);
}
