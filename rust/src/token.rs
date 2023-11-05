#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    EoF,
    Identifier(String),
    Int(i64),
    Assign,
    Plus,
    Bang,
    Minus,
    Slash,
    Asterisk,
    LessThan,
    GreaterThan,
    Equals,
    NotEquals,
    Comma,
    Semicolon,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Keyword(Keyword),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Func,
    Let,
    Return,
    If,
    Else,
    True,
    False,
}

impl Token {
    pub fn from_identifier(s: &str) -> Self {
        match s {
            "let" => Token::Keyword(Keyword::Let),
            "fn" => Token::Keyword(Keyword::Func),
            "return" => Token::Keyword(Keyword::Return),
            "if" => Token::Keyword(Keyword::If),
            "else" => Token::Keyword(Keyword::Else),
            "true" => Token::Keyword(Keyword::True),
            "false" => Token::Keyword(Keyword::False),
            _ => Token::Identifier(s.to_string()),
        }
    }
}
