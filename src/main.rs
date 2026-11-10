mod error;
mod lexer;
mod token;

use token::TokenKind;

fn main() {
    let src = "priv x = 1 if x and not привет_1 else true";
    let mut lexer = lexer::Lexer::new(src);
    loop {
        match lexer.next_token() {
            Ok(t) => {
                println!("{:?}", t);
                if t.kind == TokenKind::Eof {
                    break;
                }
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}
