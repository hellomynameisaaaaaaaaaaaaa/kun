mod lexer;
mod util;
mod parser;

fn main() {
    parser::parser(String::from(r"
        foo: string = 'hello world';
        bar: number = 20;
    "))
}
