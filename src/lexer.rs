use logos::{Logos, Lexer};

fn number(lexer: &mut Lexer<Token>) -> Option<f64> {
    String::from(lexer.slice()).parse::<f64>().ok()
}

fn word(lexer: &mut Lexer<Token>) -> Option<String> {
    Some(String::from(lexer.slice()))
}

fn flag(lexer: &mut Lexer<Token>) -> Option<String> {
    Some(lexer.slice()[1..].to_string())
}


#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[regex(r"@[A-Za-z]+", flag)]
    Flag(String),

    #[regex(r"[\-]?[0-9]+[\.]?[0-9]*", number)]
    Number(f64),

    #[regex(r"=")]
    Assign,

    #[regex(r":")]
    Type,

    #[regex(r"[A-Za-z][A-Za-z0-9]*", word)]
    Word(String),

    #[regex(r";")]
    Delimeter,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error
}