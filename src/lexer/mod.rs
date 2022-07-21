use logos::{Logos, Lexer};
use crate::token::Token;

pub fn lex(contents: &str) -> Lexer<Token> {
    Token::lexer(contents)
}