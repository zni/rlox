use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::process;

#[derive(Debug, Clone)]
enum TokenType {
    // Single character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF
}

struct Scanner {
    source: Vec<char>,
    tokens: Vec<TokenType>,
    reserved: HashMap<String, TokenType>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    fn new(source: Vec<char>) -> Scanner {
        let mut reserved = HashMap::new();
        reserved.insert(String::from("and"), TokenType::And);
        reserved.insert(String::from("class"), TokenType::Class);
        reserved.insert(String::from("else"), TokenType::Else);
        reserved.insert(String::from("false"), TokenType::False);
        reserved.insert(String::from("for"), TokenType::For);
        reserved.insert(String::from("fun"), TokenType::Fun);
        reserved.insert(String::from("if"), TokenType::If);
        reserved.insert(String::from("nil"), TokenType::Nil);
        reserved.insert(String::from("or"), TokenType::Or);
        reserved.insert(String::from("print"), TokenType::Print);
        reserved.insert(String::from("return"), TokenType::Return);
        reserved.insert(String::from("super"), TokenType::Super);
        reserved.insert(String::from("this"), TokenType::This);
        reserved.insert(String::from("true"), TokenType::True);
        reserved.insert(String::from("var"), TokenType::Var);
        reserved.insert(String::from("while"), TokenType::While);

        Scanner {
            source,
            tokens: Vec::new(),
            reserved,
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(TokenType::EOF);
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.tokens.push(TokenType::LeftParen),
            ')' => self.tokens.push(TokenType::RightParen),
            '{' => self.tokens.push(TokenType::LeftBrace),
            '}' => self.tokens.push(TokenType::RightBrace),
            ',' => self.tokens.push(TokenType::Comma),
            '.' => self.tokens.push(TokenType::Dot),
            '-' => self.tokens.push(TokenType::Minus),
            '+' => self.tokens.push(TokenType::Plus),
            ';' => self.tokens.push(TokenType::Semicolon),
            '*' => self.tokens.push(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.tokens.push(TokenType::BangEqual);
                } else {
                    self.tokens.push(TokenType::Bang);
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.tokens.push(TokenType::EqualEqual);
                } else {
                    self.tokens.push(TokenType::Equal);
                }
            },
            '<' => {
                if self.match_char('=') {
                    self.tokens.push(TokenType::LessEqual);
                } else {
                    self.tokens.push(TokenType::Less);
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.tokens.push(TokenType::GreaterEqual);
                } else {
                    self.tokens.push(TokenType::Greater);
                }
            },
            '/' => {
                if self.match_char('/') {
                    while (self.peek() != '\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.tokens.push(TokenType::Slash);
                }
            },
            ' ' => return,
            '\t' => return,
            '\r' => return,
            '\n' => {
                self.line += 1;
                return;
            },
            '"' => self.string(),
            _   => {
                if Scanner::is_digit(c) {
                    self.number();
                } else if Scanner::is_alpha(c) {
                    self.identifier();
                } else {
                    self.error("Unknown character");
                }
            },
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.source.len() {
            return '\0';
        }

        return self.source[self.current + 1];
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string.");
            return;
        }

        self.advance();
        let slice: Vec<char> = self.source[self.start + 1..self.current - 1].to_vec();
        let slice: String = slice.iter().collect();
        self.tokens.push(TokenType::String(slice));
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Scanner::is_digit(self.peek_next()) {
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let slice: Vec<char> = self.source[self.start..self.current].to_vec();
        let slice: String = slice.iter().collect();
        let digit: f64 = match slice.parse() {
            Ok(d) => d,
            Err(_) => {
                self.error("Failed to parse digit");
                0.0
            }
        };
        self.tokens.push(TokenType::Number(digit));
    }

    fn identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let slice: Vec<char> = self.source[self.start..self.current].to_vec();
        let slice: String = slice.iter().collect();

        match self.reserved.get(&slice) {
            Some(t) => self.tokens.push(t.clone()),
            None => self.tokens.push(TokenType::Identifier(slice)),
        }
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_'
    }

    fn is_alpha_numeric(c: char) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }

    fn error(&self, message: &str) {
        println!("Error at line {}, {}", self.line, message);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: scanner <file>");
        process::exit(1);
    }

    let path = Path::new(&args[1]);
    let mut file = File::open(&path)
        .expect("Failed to open file");

    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("Failed to read file");

    let source: Vec<char> = source.chars().collect();
    let mut scanner: Scanner = Scanner::new(source);
    scanner.scan_tokens();
    println!("{:?}", scanner.tokens);
}

