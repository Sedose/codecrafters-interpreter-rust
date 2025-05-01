use crate::interpreter::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub fn comment_rule(character_iterator: &mut Peekable<Chars>) -> Option<Token> {
    if *character_iterator.peek()? != '/' {
        return None;
    }
    let mut clone = character_iterator.clone();
    clone.next();
    if clone.peek() != Some(&'/') {
        return None;
    }
    character_iterator.next();
    character_iterator.next();
    while let Some(&ch) = character_iterator.peek() {
        if ch == '\n' {
            break;
        }
        character_iterator.next();
    }
    None
}
