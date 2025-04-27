mod interpreter;
use interpreter::token::Token;
use interpreter::token_type::TokenType;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

struct ScanResult {
    tokens: Vec<Token>,
    encountered_lexical_error: bool,
}

fn main() {
    let input_arguments: Vec<String> = env::args().collect();
    if input_arguments.len() < 3 {
        writeln!(
            io::stderr(),
            "Usage: {} tokenize <filename>",
            input_arguments[0]
        )
            .unwrap();
        return;
    }
    let command_name = &input_arguments[1];
    let file_name = &input_arguments[2];
    if command_name != "tokenize" {
        writeln!(io::stderr(), "Unknown command: {}", command_name).unwrap();
        return;
    }

    let file_contents = fs::read_to_string(file_name).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {file_name}").unwrap();
        String::new()
    });

    let ScanResult { tokens, encountered_lexical_error } = scan_tokens(&file_contents);

    for token in tokens {
        println!("{} {} null", token.token_type.as_output(), token.lexeme);
    }

    if encountered_lexical_error {
        process::exit(65);
    }
}

fn scan_tokens(source_text: &str) -> ScanResult {
    let mut tokens: Vec<Token> = Vec::new();
    let mut encountered_lexical_error = false;

    for current_character in source_text.chars() {
        match current_character {
            '(' => tokens.push(Token::new(TokenType::LeftParen, "(")),
            ')' => tokens.push(Token::new(TokenType::RightParen, ")")),
            '{' => tokens.push(Token::new(TokenType::LeftBrace, "{")),
            '}' => tokens.push(Token::new(TokenType::RightBrace, "}")),
            ',' => tokens.push(Token::new(TokenType::Comma, ",")),
            '.' => tokens.push(Token::new(TokenType::Dot, ".")),
            '-' => tokens.push(Token::new(TokenType::Minus, "-")),
            '+' => tokens.push(Token::new(TokenType::Plus, "+")),
            ';' => tokens.push(Token::new(TokenType::Semicolon, ";")),
            '*' => tokens.push(Token::new(TokenType::Star, "*")),
            unexpected_character => {
                eprintln!(
                    "[line 1] Error: Unexpected character: {}",
                    unexpected_character
                );
                encountered_lexical_error = true;
            }
        }
    }

    tokens.push(Token::new(TokenType::Eof, ""));
    ScanResult { tokens, encountered_lexical_error }
}
