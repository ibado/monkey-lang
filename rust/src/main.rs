mod lexer;
mod token;

use lexer::Lexer;
use std::io::Write;
use token::Token;

fn main() {
    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let code = std::io::stdin().lines().take(1).flatten().last().unwrap();
        let mut lexer = Lexer::new(code);
        while let Ok(token) = lexer.next_token() {
            if token == Token::EoF {
                break;
            }
            println!("{:?}", token);
        }
    }
}
