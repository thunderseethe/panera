#[macro_use]
extern crate nom;
extern crate regex;

mod ast;
mod token;
mod parser;
mod lexer;

use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use token::Token;




fn main() {
    let mut f = File::open("/Users/dead/Dropbox/code/rust/panera/test.pn").unwrap();
    let mut buf: Vec<u8> = Vec::new();

    f.read_to_end(&mut buf).unwrap();
    let buf: String = String::from_utf8(buf).unwrap();

    //let rules: Vec<(&str, Box<Fn(String) -> Option<Token>>)> =

    let tokens: Vec<Token> = lexer::lex(buf, lexer::token_rules());
    let ast = parser::parse(&tokens[..]);
    println!("{:?}", ast);
    //println!("{:?}", tokens);
}
