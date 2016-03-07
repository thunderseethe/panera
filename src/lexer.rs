use std::fmt::Debug;
use std::vec::Vec;
use regex::Regex;
use token::Token;

pub fn lex<O: Debug>(i: String, regex: Vec<(&str, Box<Fn(String) -> Option<O>>)>) -> Vec<O> {
    let rules: Vec<(Regex, Box<Fn(String) -> Option<O>>)> =
        regex.into_iter()
             .map(|(string, action)| (Regex::new(&(["^", string].concat())[..]).unwrap(), action))
             .collect();

    let mut input = i;
    let mut tokens: Vec<O> = Vec::new();

    while input.len() != 0 {

        let check = input.clone();

        for &(ref rule, ref action) in rules.iter() {
            match rule.find(&input[..]) {
                None => { continue },
                Some((start, end)) => {
                    if start != 0 { continue } //Only match rules at start of input
                    let (token, rest) = {
                        let (t, r) = input.split_at(end);
                        (String::from(t), String::from(r))
                    };
                    match action(token) {
                        Some(tok) => { tokens.push(tok); },
                        None => {},
                    }
                    input = rest;
                }
            }
        }

        if check.len() == input.len() { println!("{:?} == {:?}", check, input); panic!(format!("No rules matched input {:?}", input)) }
    }

    return tokens;
}

#[inline]
pub fn token_rules() -> Vec<(&'static str, Box<Fn(String) -> Option<Token>>)> {
    vec![
        ("[!#$%&\\*\\+\\./<>\\?@\\\\\\^\\|~:-]+", Box::new(|token| Some(Token::Operator(String::from(token))))),
        ("[a-zA-Z][a-zA-Z0-9\']*", Box::new(|token| Some(Token::Identifier(String::from(token))))),
        ("0|[1-9][0-9]*", Box::new(|token| Some(Token::Int(token.parse().unwrap())))),
        ("\\(", Box::new(|_| Some(Token::LeftParen))),
        ("\\)", Box::new(|_| Some(Token::RightParen))),
        ("=", Box::new(|_| Some(Token::Define))),
        ("`", Box::new(|_| Some(Token::BackTick))),
        ("\n+", Box::new(|_| Some(Token::NewLine))),
        ("\\s+", Box::new(|_| None)),
    ]
}
