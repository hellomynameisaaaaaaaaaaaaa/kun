mod lexer;
mod util;
mod parser;

fn main() {
    parser::parser(String::from(r"
        foo = 10;
        bar: number = 20;
    "))
}
