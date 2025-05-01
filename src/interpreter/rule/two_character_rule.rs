use crate::interpreter::token::Token;
use crate::interpreter::token_type::TokenType;
use crate::interpreter::token_type::TokenType::{BangEqual, EqualEqual, GreaterEqual, LessEqual};
use std::iter::Peekable;
use std::str::Chars;

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
