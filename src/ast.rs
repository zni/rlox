use crate::scanner;

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, scanner::Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(scanner::Token),
    Unary(scanner::Token, Box<Expr>),
}
