use crate::interpreter::token::Token;
use crate::interpreter::token_type::TokenType;
use crate::interpreter::token_type::TokenType::*;
use std::iter::Peekable;
use std::str::Chars;

pub fn single_character_rule(character_iterator: &mut Peekable<Chars>) -> Option<Token> {
    let current_character = *character_iterator.peek()?;
    let (lexeme, token_type) = single_character_token(current_character)?;
    character_iterator.next();
    Some(Token::new(token_type, lexeme))
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
