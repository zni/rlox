use crate::scanner;

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, scanner::Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(scanner::Token),
    Unary(scanner::Token, Box<Expr>),
}

/*
 * expression     → equality ;
 * equality       → comparison ( ( "!=" | "==" ) comparison )* ;
 * comparison     → addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
 * addition       → multiplication ( ( "-" | "+" ) multiplication )* ;
 * multiplication → unary ( ( "/" | "*" ) unary )* ;
 * unary          → ( "!" | "-" ) unary
 *                | primary ;
 * primary        → NUMBER | STRING | "false" | "true" | "nil"
 *                | "(" expression ")" ;
 */

pub struct Parser {
    current: usize,
    tokens: Vec<scanner::Token>,
}

impl Parser {
    pub fn new(tokens: Vec<scanner::Token>) -> Parser {
        Parser {
            current: 0,
            tokens
        }
    }

    pub fn parse(&mut self) -> Result<Expr, &'static str> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, &'static str> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.comparison();
        while self.match_token(vec![scanner::TokenType::BangEqual,
                                    scanner::TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            if right.is_ok() && expr.is_ok() {
                expr = Ok(Expr::Binary(Box::new(expr.unwrap()), operator, Box::new(right.unwrap())));
            } else {
                return Err("expecting equality");
            }
        }

        expr
    }

    fn comparison(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.addition();
        while self.match_token(vec![scanner::TokenType::Greater,
                                    scanner::TokenType::GreaterEqual,
                                    scanner::TokenType::Less,
                                    scanner::TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.addition();
            if right.is_ok() && expr.is_ok() {
                expr = Ok(Expr::Binary(Box::new(expr.unwrap()), operator, Box::new(right.unwrap())));
            } else {
                return Err("expecting comparison");
            }
        }

        expr
    }

    fn addition(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.multiplication();
        while self.match_token(vec![scanner::TokenType::Plus,
                                    scanner::TokenType::Minus]) {
            let operator = self.previous();
            let right = self.multiplication();
            if right.is_ok() && expr.is_ok() {
                expr = Ok(Expr::Binary(Box::new(expr.unwrap()), operator, Box::new(right.unwrap())));
            } else {
                return Err("expecting addition");
            }
        }

        expr
    }

    fn multiplication(&mut self) -> Result<Expr, &'static str> {
        let mut expr = self.unary();
        while self.match_token(vec![scanner::TokenType::Star,
                                    scanner::TokenType::Slash]) {
            let operator = self.previous();
            let right = self.unary();
            if right.is_ok() && expr.is_ok() {
                expr = Ok(Expr::Binary(Box::new(expr.unwrap()), operator, Box::new(right.unwrap())));
            } else {
                return Err("expecting multiplication");
            }
        }

        expr
    }

    fn unary(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![scanner::TokenType::Bang,
                                 scanner::TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            if right.is_ok() {
                return Ok(Expr::Unary(operator, Box::new(right.unwrap())));
            } else {
                return Err("expecting unary");
            }
        }

        let primary = self.primary();
        if primary.is_ok() {
            return Ok(primary.unwrap());
        } else {
            return Err("expecting primary");
        }
    }

    fn primary(&mut self) -> Result<Expr, &'static str> {
        if self.match_token(vec![scanner::TokenType::False]) {
            return Ok(Expr::Literal(self.previous()));
        }
        if self.match_token(vec![scanner::TokenType::True]) {
            return Ok(Expr::Literal(self.previous()));
        }
        if self.match_token(vec![scanner::TokenType::Nil]) {
            return Ok(Expr::Literal(self.previous()));
        }

        if let scanner::TokenType::Number(_) = self.peek().token_type {
            self.advance();
            return Ok(Expr::Literal(self.previous()));
        }

        if let scanner::TokenType::String(_) = self.peek().token_type {
            self.advance();
            return Ok(Expr::Literal(self.previous()));
        }

        if self.match_token(vec![scanner::TokenType::LeftParen]) {
            let expr = self.expression();

            let consumed = self.consume(scanner::TokenType::RightParen, "Expect ')' after expression");
            if consumed.is_err() {
                return Err("Expect ')' after expression");
            }

            if expr.is_ok() {
                return Ok(Expr::Grouping(Box::new(expr.unwrap())));
            } else {
                return Err("expecting grouping");
            }
        }

        Err("expecting expression")
    }

    fn consume(&mut self, token_type: scanner::TokenType, message: &'static str) -> Result<scanner::Token, &'static str> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        Err(message)
    }

    fn match_token(&mut self, tokens: Vec<scanner::TokenType>) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn check(&mut self, token_type: scanner::TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        let token = self.peek();
        token.token_type == token_type
    }

    fn advance(&mut self) -> scanner::Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        let token = self.peek();
        token.token_type == scanner::TokenType::EOF
    }

    fn peek(&self) -> scanner::Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> scanner::Token {
        self.tokens[self.current - 1].clone()
    }
}
