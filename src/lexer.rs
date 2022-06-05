use logos::{Logos, Lexer};

fn number(lexer: &mut Lexer<Token>) -> Option<f64> {
    String::from(lexer.slice()).parse::<f64>().ok()
}

fn string(lexer: &mut Lexer<Token>) -> Option<String> {
    Some(lexer.slice()[1..String::from(lexer.slice).len() - 1])
}

fn word(lexer: &mut Lexer<Token>) -> Option<String> {
    Some(String::from(lexer.slice()))
}

fn flag(lexer: &mut Lexer<Token>) -> Option<String> {
    Some(lexer.slice()[1..].to_string())
}

fn types(lexer: &mut Lexer<Token>) -> Option<Types> {
    match lexer.slice() {
        "number" => Some(Types::Number),
        "string" => Some(Types::String),

        _ => None
    }
}

#[derive(Debug)]
enum Flag {
    Use,
    Error
}

#[derive(Debug, PartialEq)]
pub enum Types {
    Number,
    String,
    None
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
    SetType,

    #[regex(r"'.*'", string)]
    String(String),

    #[regex(r"[A-Za-z][A-Za-z0-9]*", word)]
    Word(String),

    #[regex(r"(number|string)", types)]
    Type(Types),

    #[regex(r";")]
    Delimeter,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error
}