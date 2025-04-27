mod interpreter;
use interpreter::token::Token;
use interpreter::token_type::TokenType;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

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

    let ScanResult {
        tokens,
        encountered_lexical_error,
    } = scan_tokens(&file_contents);

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
    let mut characters = source_text.chars().peekable();

    while let Some(current_character) = characters.next() {
        match current_character {
            '=' => {
                if let Some('=') = characters.peek() {
                    characters.next();
                    tokens.push(Token::new(TokenType::EqualEqual, "=="));
                } else {
                    tokens.push(Token::new(TokenType::Equal, "="));
                }
            }
            '!' => {
                if let Some('=') = characters.peek() {
                    characters.next();
                    tokens.push(Token::new(TokenType::BangEqual, "!="));
                } else {
                    tokens.push(Token::new(TokenType::Bang, "!"));
                }
            }
            other_character => {
                if let Some((lexeme, token_type)) = single_character_token(other_character) {
                    tokens.push(Token::new(token_type, lexeme));
                } else {
                    eprintln!("[line 1] Error: Unexpected character: {}", other_character);
                    encountered_lexical_error = true;
                }
            }
        }
    }

    tokens.push(Token::new(TokenType::Eof, ""));
    ScanResult {
        tokens,
        encountered_lexical_error,
    }
}

fn single_character_token(character: char) -> Option<(&'static str, TokenType)> {
    match character {
        '(' => Some(("(", TokenType::LeftParen)),
        ')' => Some((")", TokenType::RightParen)),
        '{' => Some(("{", TokenType::LeftBrace)),
        '}' => Some(("}", TokenType::RightBrace)),
        ',' => Some((",", TokenType::Comma)),
        '.' => Some((".", TokenType::Dot)),
        '-' => Some(("-", TokenType::Minus)),
        '+' => Some(("+", TokenType::Plus)),
        ';' => Some((";", TokenType::Semicolon)),
        '*' => Some(("*", TokenType::Star)),
        _ => None,
    }
}

struct ScanResult {
    tokens: Vec<Token>,
    encountered_lexical_error: bool,
}
