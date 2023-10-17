#![allow(dead_code)]
mod token;

use token::*;

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut n = Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        n.read_char();
        n
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();
        let token = match self.ch as char {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::OpenParenthesis,
            ')' => Token::CloseParenthesis,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '!' => Token::Bang,
            '-' => Token::Minus,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            '\0' => Token::EoF,
            x if is_letter(x) => self.read_identifier()?,
            x if is_digit(x) => self.read_number()?,
            _ => Token::Illegal,
        };
        match token {
            Token::Keyword(_) | Token::Identifier(_) | Token::Int(_) => (),
            _ => self.read_char()
        }

        Ok(token)
    }

    fn skip_whitespace(&mut self) {
        let mut c = self.ch as char;
        while c == ' ' || c == '\n' || c == '\t' || c == '\r' {
		    self.read_char();
            c = self.ch as char;
	    }
    }

    fn read_number(&mut self) ->Result<Token, String> {
        let position = self.position;
        while is_digit(self.ch as char) {
            self.read_char();
        }

        let bytes = &self.input.as_bytes()[position..self.position];
        let identifier: &str = std::str::from_utf8(bytes)
            .map_err(|e| format!("Invalid integer: {:?}. {e}", bytes))?;
        let int = identifier.parse().map_err(|e| format!("{e}"))?;

        Ok(Token::Int(int))

    }

    fn read_identifier(&mut self) -> Result<Token, String> {
        let position = self.position;
        while is_letter(self.ch as char) {
            self.read_char();
        }

        let bytes = &self.input.as_bytes()[position..self.position];
        let identifier: &str = std::str::from_utf8(bytes)
            .map_err(|e| format!("Invalid identifier: {:?}. {e}", bytes))?;

        Ok(Token::from_identifier(identifier))
    }

    fn read_char(&mut self) {
        self.ch = if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        };

        self.position = self.read_position;
        self.read_position += 1;
    }
}

fn is_digit(c: char) -> bool {
    '0' <= c && '9' >= c
}

fn is_letter(c: char) -> bool {
    'a' <= c && 'z' >= c || 'A' <= c && 'Z' >= c || c == '_'
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn next_token_first_test() {
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
	        5 < 10 > 5;
            return if (true) 1 else 2
        "#;
        let mut lexer = Lexer::new(input.to_string());
        let expected_tokens = vec![
            Token::Keyword(Keyword::Let),
            Token::Identifier("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Keyword(Keyword::Let),
            Token::Identifier("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Keyword(Keyword::Let),
            Token::Identifier("add".to_string()),
            Token::Assign,
            Token::Keyword(Keyword::Func),
            Token::OpenParenthesis,
            Token::Identifier("x".to_string()),
            Token::Comma,
            Token::Identifier("y".to_string()),
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Semicolon,
            Token::CloseBrace,
            Token::Semicolon,
            Token::Keyword(Keyword::Let),
            Token::Identifier("result".to_string()),
            Token::Assign,
            Token::Identifier("add".to_string()),
            Token::OpenParenthesis,
            Token::Identifier("five".to_string()),
            Token::Comma,
            Token::Identifier("ten".to_string()),
            Token::CloseParenthesis,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::LessThan,
            Token::Int(10),
            Token::GreaterThan,
            Token::Int(5),
            Token::Semicolon,
            Token::Keyword(Keyword::Return),
            Token::Keyword(Keyword::If),
            Token::OpenParenthesis,
            Token::Keyword(Keyword::True),
            Token::CloseParenthesis,
            Token::Int(1),
            Token::Keyword(Keyword::Else),
            Token::Int(2),
            Token::EoF,
        ];

        for (i, _) in expected_tokens.iter().enumerate() {
            assert_eq!(expected_tokens[i], lexer.next_token().unwrap(), "case[{i}]");
        }
    }
}
