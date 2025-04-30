mod interpreter;

use interpreter::token::Token;
use interpreter::token_type::TokenType;
use interpreter::token_type::TokenType::*;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::iter::Peekable;
use std::process;
use std::str::Chars;

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
    let mut character_iterator = source.chars().peekable();

    while let Some(_) = character_iterator.peek() {
        let maybe_token = RULE_FUNCTIONS
          .iter()
          .find_map(|rule| rule(&mut character_iterator));

        if let Some(token) = maybe_token {
            if token.token_type != Comment {
                tokens.push(token);
            }
        } else if let Some(unexpected_character) = character_iterator.next() {
            eprintln!(
                "[line 1] Error: Unexpected character: {}",
                unexpected_character
            );
            encountered_lexical_error = true;
        }
    }

    tokens.push(Token::new(Eof, ""));
    ScanResult {
        tokens,
        encountered_lexical_error,
    }
}

fn single_character_token(character: char) -> Option<(&'static str, TokenType)> {
    let res = match character {
        '(' => ("(", LeftParen),
        ')' => (")", RightParen),
        '{' => ("{", LeftBrace),
        '}' => ("}", RightBrace),
        ',' => (",", Comma),
        '.' => (".", Dot),
        '-' => ("-", Minus),
        '+' => ("+", Plus),
        ';' => (";", Semicolon),
        '*' => ("*", Star),
        '=' => ("=", Equal),
        '!' => ("!", Bang),
        '<' => ("<", Less),
        '>' => (">", Greater),
        _ => return None,
    };
    Some(res)
}

fn two_character_token(
    first: char,
    second: Option<&char>,
) -> Option<(TokenType, &'static str)> {
    let token_pair = match (first, *second?) {
        ('=', '=') => (EqualEqual, "=="),
        ('!', '=') => (BangEqual, "!="),
        ('<', '=') => (LessEqual, "<="),
        ('>', '=') => (GreaterEqual, ">="),
        _ => return None,
    };
    Some(token_pair)
}

struct ScanResult {
    tokens: Vec<Token>,
    encountered_lexical_error: bool,
}

static RULE_FUNCTIONS: [fn(&mut Peekable<Chars>) -> Option<Token>; 3] = [
    comment_rule,
    two_character_rule,
    single_character_rule,
];

fn comment_rule(char_iterator: &mut Peekable<Chars>) -> Option<Token> {
    if char_iterator.peek().copied() != Some('/') {
        return None;
    }
    char_iterator.next();
    if char_iterator.peek().copied() != Some('/') {
        return Some(Token::new(Slash, "/"));
    }
    char_iterator.next();
    for next_character in char_iterator {
        if next_character == '\n' {
            break;
        }
    }
    None
}

pub fn two_character_rule(character_iterator: &mut Peekable<Chars>) -> Option<Token> {
    let current_character = *character_iterator.peek()?;
    let mut ahead_iterator = character_iterator.clone();
    ahead_iterator.next();
    let next_character = *ahead_iterator.peek()?;
    let (token_type, lexeme) = two_character_token(current_character, Some(&next_character))?;
    character_iterator.next();
    character_iterator.next();
    Some(Token::new(token_type, lexeme))
}

pub fn single_character_rule(character_iterator: &mut Peekable<Chars>) -> Option<Token> {
    let current_character = *character_iterator.peek()?;
    let (lexeme, token_type) = single_character_token(current_character)?;
    character_iterator.next();
    Some(Token::new(token_type, lexeme))
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
    fn test_relational_operators() {
        let (outputs, error) = lex_outputs("<<=>>=");
        assert!(!error);
        assert_eq!(
            outputs,
            vec![
                "LESS < null",
                "LESS_EQUAL <= null",
                "GREATER > null",
                "GREATER_EQUAL >= null",
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
