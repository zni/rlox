use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::process;

#[derive(Debug)]
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

    // Keywords
    If,
    Then,
    Else
}

fn peek(s: &mut String) -> char {
    let mut iter = s.chars();
    match iter.next() {
        Some(m) => {
            return m;
        },
        None => return '\0',
    }
}

fn match_char(s: &mut String, c: char) -> bool {
    let mut iter = s.chars();
    match iter.next() {
        Some(m) => {
            s.remove(0);
            return m == c;
        },
        None => return false,
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

    // let mut source: Vec<char> = source.chars().collect()

    let mut tokens: Vec<TokenType> = Vec::new();
    while !source.is_empty() {
        let c = source.remove(0);

        if c.is_whitespace() {
            continue;
        }

        match c {
            '(' => tokens.push(TokenType::LeftParen),
            ')' => tokens.push(TokenType::RightParen),
            '{' => tokens.push(TokenType::LeftBrace),
            '}' => tokens.push(TokenType::RightBrace),
            ',' => tokens.push(TokenType::Comma),
            '.' => tokens.push(TokenType::Dot),
            '-' => tokens.push(TokenType::Minus),
            '+' => tokens.push(TokenType::Plus),
            ';' => tokens.push(TokenType::Semicolon),
            '*' => tokens.push(TokenType::Star),
            '!' => tokens.push(
                if match_char(&mut source, '=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }),
            '=' => tokens.push(
                if match_char(&mut source, '=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }),
            '<' => tokens.push(
                if match_char(&mut source, '=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Equal
                }),
            '>' => tokens.push(
                if match_char(&mut source, '=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Equal
                }),
            _   => continue,
        }
    }

    println!("{:?}", tokens);
}
