#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Eof,
}

impl TokenType {
    pub fn as_output(&self) -> &'static str {
        match self {
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::Eof => "EOF",
        }
    }
}
