use crate::lexer::Token;
use logos::{Logos, Lexer};

enum Types<'a> {
    Number(f64),
    String(String),
    Array(&'a Types<'a>)
}
enum Sentences<'a> {
    Define(String, Types<'a>)
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
}

fn printl(mut lexer: Lexer<Token>, n: u8, mut a: Vec<Token>) -> Vec<Token> {
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