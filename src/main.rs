use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::env;
use std::path::Path;
use std::process;

mod scanner;
mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("usage: rlox <file>");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        run_prompt();
    }
}

fn run_file(file: &String) {
    let path = Path::new(file);
    let mut file = File::open(&path)
        .expect("Failed to open file");

    let mut source = String::new();
    file.read_to_string(&mut source)
        .expect("Failed to read file");

    let source: Vec<char> = source.chars().collect();
    run(source);
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush()
            .expect("Failed to flush output");

        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");
        let line: Vec<char> = line.chars().collect();
        run(line);
    }
}

fn run(source: Vec<char>) {
    let mut scanner: scanner::Scanner = scanner::Scanner::new(source);
    scanner.scan_tokens();
    println!("{:?}", scanner.tokens);

    let mut parser: ast::Parser = ast::Parser::new(scanner.tokens);
    println!("{:?}", parser.parse());
}
