use crate::interpreter::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub fn comment_rule(
    character_iterator: &mut Peekable<Chars>,
    line_number: &mut usize,
) -> Option<Token> {
    if character_iterator.peek() != Some(&'/') {
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
        character_iterator.next();
        if ch == '\n' {
            *line_number += 1;
            break;
        }
    }
    None
}
