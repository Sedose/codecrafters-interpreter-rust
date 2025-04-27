mod interpreter;
use interpreter::token::Token;
use interpreter::token_type::TokenType;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let cli_arguments: Vec<String> = env::args().collect();
    if cli_arguments.len() < 3 {
        writeln!(
            io::stderr(),
            "Usage: {} tokenize <filename>",
            cli_arguments[0],
        )
        .unwrap();
        return;
    }

    let command = &cli_arguments[1];
    let filename = &cli_arguments[2];

    if command != "tokenize" {
        writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
        return;
    }

    let source_text = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {filename}").unwrap();
        String::new()
    });

    let scanned_tokens = scan_tokens(&source_text);

    for token in scanned_tokens {
        println!("{} {} null", token.token_type.as_output(), token.lexeme);
    }
}

fn scan_tokens(source_text: &str) -> Vec<Token> {
    let mut scanned_tokens: Vec<Token> = source_text
        .chars()
        .filter_map(|current_char| match current_char {
            '(' => Some(Token::new(TokenType::LeftParen, "(")),
            ')' => Some(Token::new(TokenType::RightParen, ")")),
            '{' => Some(Token::new(TokenType::LeftBrace, "{")),
            '}' => Some(Token::new(TokenType::RightBrace, "}")),
            _ => None,
        })
        .collect();

    scanned_tokens.push(Token::new(TokenType::Eof, ""));
    scanned_tokens
}
