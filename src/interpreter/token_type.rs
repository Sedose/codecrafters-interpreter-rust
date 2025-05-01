#[derive(Debug)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Eof,
}

impl TokenType {
    pub fn as_output(&self) -> &'static str {
        use TokenType::*;
        match self {
            LeftParen => "LEFT_PAREN",
            RightParen => "RIGHT_PAREN",
            LeftBrace => "LEFT_BRACE",
            RightBrace => "RIGHT_BRACE",
            Comma => "COMMA",
            Dot => "DOT",
            Minus => "MINUS",
            Plus => "PLUS",
            Semicolon => "SEMICOLON",
            Star => "STAR",
            Bang => "BANG",
            BangEqual => "BANG_EQUAL",
            Equal => "EQUAL",
            EqualEqual => "EQUAL_EQUAL",
            Less => "LESS",
            LessEqual => "LESS_EQUAL",
            Greater => "GREATER",
            GreaterEqual => "GREATER_EQUAL",
            Eof => "EOF",
        }
    }
}
