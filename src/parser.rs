use crate::lexer::{Token, Types};
use logos::{Logos, Lexer};

#[derive(Debug)]
enum Value {
    Number(f64),
    String(String),
    None
}

#[derive(Debug)]
enum Sentences {
    Define(String, Types, Value),
    Error
}

pub fn parser(text: String) {
    let log: Vec<String> = vec![
        String::from("KUN LOG")
    ];

    let lexer: Lexer<Token> = Token::lexer(&text).into_iter();

    println!("LEXER RESULTS");
    let lexer: Vec<Token> = printl(lexer, 0, vec![]);

    println!("LEXER GROUPINGS");
    let lexer_group: Vec<Vec<Token>> = split_lexer(lexer);

    let mut parsed: Vec<Sentences> = vec![];
    for i in lexer_group {
        parsed.push(match i {
            vec![Token::Word(x), Token::Assign, Token::Number(y)] => Sentences::Define(x, Types::Number, Value::Number(y)), // x = y; where y is a valid number
            vec![Token::Word(x), Token::SetType, Token::Type(Types::Number), Token::Number(y)] => Sentences::Define(x, Types::Number, Value::Number(y)), // x: number = y;
            vec![Token::Word(x), Token::Assign, Token::String(y)] => Sentences::Define(x, Types::String, Value::String(y)), // x = y; where y is a valid string
            vec![Token::Word(x), Token::SetType, Token::Type(Types::String), Token::String(y)] => Sentences::Define(x, Types::String, Value::String(y)), // x: string = y;
            _ => Sentences::Error
        })

    }
    println!("{:#?}", parsed);
}

fn printl(mut lexer: Lexer<Token>, n: u8, mut a: Vec<Token>) -> Vec<Token> { // TODO make this into lexer.collect()
    if let Some(l) = lexer.next() {
        println!("{} {:?}", format!("{:0>2x}", n), l);
        a.push(l);
        return printl(lexer, n+1, a);
    } else {
        return a;
    }
}

fn split_lexer(mut lexer:Vec<Token>) -> Vec<Vec<Token>> {
    let mut out: Vec<Vec<Token>> = vec![vec![]];
    let mut n = 0;
    for i in lexer {
        if i == Token::Delimeter {
            n += 1;
            out.push(vec![]);
        } else if i != Token::Error {
            out[n].push(i);
        } else {
            panic!("lexer error: see lexer results log for more info. triggered from the fn <split_lexer> in parser.rs")
        }
    }
    println!("{:#?}", out);
    return out; 
}