use crate::interpreter::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: &'static str,
    pub literal: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &'static str) -> Self {
        Token {
            token_type,
            lexeme,
            literal: None,
        }
    }
}
