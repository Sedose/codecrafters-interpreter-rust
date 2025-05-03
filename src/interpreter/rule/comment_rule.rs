use crate::interpreter::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub fn comment_rule(
    character_iterator: &mut Peekable<Chars>,
    line_number: &mut usize,
) -> Option<Token> {
    let mut iterator_cloned = character_iterator.clone();

    let next_two_characters: Vec<char> = iterator_cloned.by_ref().take(2).collect();

    if next_two_characters != ['/', '/'] {
        return None;
    }

    character_iterator.next();
    character_iterator.next();

    while let Some(character) = character_iterator.peek() {
        if *character == '\n' {
            character_iterator.next();
            *line_number += 1;
            break;
        }
        character_iterator.next();
    }
    None
}
