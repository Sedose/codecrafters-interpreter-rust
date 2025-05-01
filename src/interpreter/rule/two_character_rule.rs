use crate::interpreter::token::Token;
use crate::interpreter::token_type::TokenType;
use crate::interpreter::token_type::TokenType::{BangEqual, EqualEqual, GreaterEqual, LessEqual};
use std::iter::Peekable;
use std::str::Chars;

pub fn two_character_rule(
    character_iterator: &mut Peekable<Chars>,
    _line_number: &mut usize,
) -> Option<Token> {
    let first_character = match character_iterator.peek() {
        Some(character) => *character,
        None => return None,
    };

    let mut lookahead_iterator = character_iterator.clone();
    lookahead_iterator.next();

    let second_character = match lookahead_iterator.peek() {
        Some(character) => *character,
        None => return None,
    };

    let (token_type, lexeme) = match two_character_token(first_character, Some(&second_character)) {
        Some(token) => token,
        None => return None,
    };

    character_iterator.next();
    character_iterator.next();

    Some(Token::new(token_type, lexeme))
}

fn two_character_token(first: char, second: Option<&char>) -> Option<(TokenType, &'static str)> {
    let token_pair = match (first, *second?) {
        ('=', '=') => (EqualEqual, "=="),
        ('!', '=') => (BangEqual, "!="),
        ('<', '=') => (LessEqual, "<="),
        ('>', '=') => (GreaterEqual, ">="),
        _ => return None,
    };
    Some(token_pair)
}
