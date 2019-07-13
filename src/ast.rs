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

struct Parser<'a> {
    current: usize,
    tokens: &'a Vec<scanner::Token>,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a Vec<scanner::Token>) -> Parser<'a> {
        Parser {
            current: 0,
            tokens
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_token(vec![scanner::TokenType::BangEqual,
                                    scanner::TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.addition();
        while self.match_token(vec![scanner::TokenType::Greater,
                                    scanner::TokenType::GreaterEqual,
                                    scanner::TokenType::Less,
                                    scanner::TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.addition();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();
        while self.match_token(vec![scanner::TokenType::Plus,
                                    scanner::TokenType::Minus]) {
            let operator = self.previous();
            let right = self.multiplication();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_token(vec![scanner::TokenType::Star,
                                    scanner::TokenType::Slash]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(vec![scanner::TokenType::Bang,
                                 scanner::TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(operator, Box::new(right));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(vec![scanner::TokenType::False]) {
            return Expr::Literal(self.previous());
        }
        if self.match_token(vec![scanner::TokenType::True]) {
            return Expr::Literal(self.previous());
        }
        if self.match_token(vec![scanner::TokenType::Nil]) {
            return Expr::Literal(self.previous());
        }

        if let scanner::TokenType::Number(_) = self.peek().token_type {
            self.advance();
            return Expr::Literal(self.previous());
        }

        if let scanner::TokenType::String(_) = self.peek().token_type {
            self.advance();
            return Expr::Literal(self.previous());
        }

        if self.match_token(vec![scanner::TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(scanner::TokenType::RightParen, "Expect ')' after expression");
            return Expr::Grouping(Box::new(expr));
        }
    }

    fn consume(&mut self, token_type: scanner::TokenType, message: &str) -> scanner::Token {
        if self.check(token_type) {
            return self.advance();
        }

        // Error
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
        self.tokens[self.current]
    }

    fn previous(&self) -> scanner::Token {
        self.tokens[self.current - 1]
    }
}
