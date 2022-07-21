use crate::token::Token;
use logos::Lexer;

pub fn parse(lexer: Lexer<Token>) {
    let mut lex = lexer.peekable();
    let token = lex.peek();

    match token {
        Some(Token::Let) => {}
        Some(Token::If) => {}
        _ => {}
    }
}
