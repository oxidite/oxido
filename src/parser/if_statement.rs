use crate::{
    store::Store,
    token::Token,
    util::{check_syntax, parse_ident},
};
use logos::Lexer;

pub fn parse_if_statement(mut lex: Lexer<Token>, mut store: Store) -> Store {
    // TOKEN: if
    check_syntax(lex.next(), Token::If, &store);

    lex.next();
    let lhs = parse_ident(&lex.slice().to_string(), &store).replace("\"", "");
    let op = lex.next().unwrap();
    lex.next();
    let rhs = parse_ident(&lex.slice().to_string(), &store).replace("\"", "");

    match op {
        Token::Equality => {
            if lhs != rhs {
                store.scopes._if += 1;
            }
        }
        Token::NotEquality => {
            if lhs == rhs {
                store.scopes._if += 1;
            }
        }
        Token::GreaterThan => {
            if lhs < rhs {
                store.scopes._if += 1;
            }
        }
        Token::LesserThan => {
            if lhs > rhs {
                store.scopes._if += 1;
            }
        }
        _ => {}
    }

    store.states.stack.push(String::from("if"));

    check_syntax(lex.next(), Token::CurlyBraceOpen, &store);

    store
}
