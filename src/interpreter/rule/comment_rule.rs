use crate::interpreter::token::Token;
use crate::interpreter::token_type::TokenType::Slash;
use std::iter::Peekable;
use std::str::Chars;

pub fn comment_rule(character_iterator: &mut Peekable<Chars>) -> Option<Token> {
    if character_iterator.peek().copied() != Some('/') {
        return None;
    }
    character_iterator.next();
    if character_iterator.peek().copied() != Some('/') {
        return Some(Token::new(Slash, "/"));
    }
    character_iterator.next();
    for next_character in character_iterator {
        if next_character == '\n' {
            break;
        }
    }
    None
}
