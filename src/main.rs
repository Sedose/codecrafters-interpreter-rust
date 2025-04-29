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

fn scan_tokens(source: &str) -> ScanResult {
    let mut tokens = Vec::new();
    let mut encountered_lexical_error = false;
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        if let Some((token_type, lexeme)) = match (c, chars.peek()) {
            ('=', Some(&'=')) => {
                chars.next();
                Some((TokenType::EqualEqual, "=="))
            }
            ('=', _) => Some((TokenType::Equal, "=")),
            ('!', Some(&'=')) => {
                chars.next();
                Some((TokenType::BangEqual, "!="))
            }
            ('!', _) => Some((TokenType::Bang, "!")),
            _ => None,
        } {
            tokens.push(Token::new(token_type, lexeme));
            continue;
        }
        if let Some((lexeme, token_type)) = single_character_token(c) {
            tokens.push(Token::new(token_type, lexeme));
        } else {
            eprintln!("[line 1] Error: Unexpected character: {}", c);
            encountered_lexical_error = true;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combined_mixed_input() {
        let (outputs, error) = lex_outputs("(,{!==}$)");
        assert!(error);
        assert_eq!(
            outputs,
            vec![
                "LEFT_PAREN ( null",
                "COMMA , null",
                "LEFT_BRACE { null",
                "BANG_EQUAL != null",
                "EQUAL = null",
                "RIGHT_BRACE } null",
                "RIGHT_PAREN ) null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_parentheses_scanning() {
        let (outputs, error) = lex_outputs("(()");
        assert!(!error);
        assert_eq!(
            outputs,
            vec![
                "LEFT_PAREN ( null",
                "LEFT_PAREN ( null",
                "RIGHT_PAREN ) null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_braces_scanning() {
        let (outputs, error) = lex_outputs("{{}}");
        assert!(!error);
        assert_eq!(
            outputs,
            vec![
                "LEFT_BRACE { null",
                "LEFT_BRACE { null",
                "RIGHT_BRACE } null",
                "RIGHT_BRACE } null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_single_character_tokens() {
        let (outputs, error) = lex_outputs("({*.,+*})");
        assert!(!error);
        assert_eq!(
            outputs,
            vec![
                "LEFT_PAREN ( null",
                "LEFT_BRACE { null",
                "STAR * null",
                "DOT . null",
                "COMMA , null",
                "PLUS + null",
                "STAR * null",
                "RIGHT_BRACE } null",
                "RIGHT_PAREN ) null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_assignment_and_equality_scanning() {
        let (outputs, error) = lex_outputs("={===}");
        assert!(!error);
        assert_eq!(
            outputs,
            vec![
                "EQUAL = null",
                "LEFT_BRACE { null",
                "EQUAL_EQUAL == null",
                "EQUAL = null",
                "RIGHT_BRACE } null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_negation_and_inequality_scanning() {
        let (outputs, error) = lex_outputs("!!===");
        assert!(!error);
        assert_eq!(
            outputs,
            vec![
                "BANG ! null",
                "BANG_EQUAL != null",
                "EQUAL_EQUAL == null",
                "EOF  null",
            ]
        );
    }

    #[test]
    fn test_unexpected_characters() {
        let (outputs, error) = lex_outputs("$#(");
        assert!(error);
        assert_eq!(outputs, vec!["LEFT_PAREN ( null", "EOF  null",]);
    }

    fn lex_outputs(source: &str) -> (Vec<String>, bool) {
        let ScanResult {
            tokens,
            encountered_lexical_error,
        } = scan_tokens(source);
        let outputs = tokens
            .into_iter()
            .map(|token| format!("{} {} null", token.token_type.as_output(), token.lexeme))
            .collect();
        (outputs, encountered_lexical_error)
    }
}
