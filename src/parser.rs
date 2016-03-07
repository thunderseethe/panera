use nom::{IResult, Err, Needed, ErrorKind, InputLength};

use std::vec::Vec;

use ast::program::Program;
use ast::function::Function;
use ast::identifier::Identifier;
use ast::expression::Expression;
use ast::value::Value;

use token::Token;

/*impl InputLength for Vec<Token> {
    pub fn input_len(&self) -> usize {
        self.len()
    }
}*/

named!(pub parse<&[Token], Program>,
    map!(
        many1!(function),
        Program::new
    )
);

named!(function<&[Token], Function>,
    chain!(
        name: function_name ~
        args: args          ~
        define              ~
        body: expression    ~
        new_line            ,
        || {
            let f = Function::new(name, args, body);
            println!("{:?}", f);
            f
        }
    )
);

named!(function_name<&[Token], Identifier>,
    alt!(
          identifier
        | delimited!(left_paren, operator, right_paren)
    )
);

named!(args< &[Token], Vec<Identifier> >,
    many0!(identifier)
);

named!(fexp<&[Token], Expression>,
    map!(
        many1!(aexp),
        |application: Vec<Expression>| { if application.len() == 1 { application[0].clone() } else { Expression::application(application) } }
        //|application:Expression| { println!("fexp {:?}", application); application }
    )
);

named!(aexp<&[Token], Expression>,
    alt!(
        delimited!(left_paren, expression, right_paren)  |
        map!(value, Expression::value)
    )
);

named!(expression<&[Token], Expression>,
    chain!(
        left: fexp      ~
        maybe_op_right:
            opt!(complete!(chain!(
                op: infix       ~
                right: expression,
                || { (op, right) }
            ))),
        || {
            match maybe_op_right.clone() {
                Some((op, right)) => Expression::application(vec![Expression::value(Value::variable(op)), left, right]),
                None => left,
            }
        }
    )
);

named!(infix<&[Token], Identifier>,
    alt!(
        operator    |
        delimited!(back_tick, identifier, back_tick)
    )
);

named!(value<&[Token], Value>,
    alt!(
          map!(int, Value::int)
        | map!(identifier, Value::variable)
    )
);

/*named!(int<i64>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(identifier<String>,
    map_res!(
        chain!(
            start: alpha ~
            rest: opt!(alphanumeric),
            || {
                match rest {
                    Some(rest) => [start, rest].concat(),
                    None => Vec::<u8>::from(start),
                }
            }
        ),
        String::from_utf8
    )
);

named!(operator<String>,
    map!(
        map_res!(is_a!("!#$%&*+./<>?@\\^|-~:"), str::from_utf8),
        String::from
    )
);*/

fn new_line(i: &[Token]) -> IResult<&[Token], char> {
    if i.len() < 1 { return IResult::Incomplete(Needed::Size(1)) }
    match i[0] {
        Token::NewLine => IResult::Done(&i[1..], '\n'),
        _ => IResult::Error(Err::Code(ErrorKind::Custom(7))),
    }
}

fn define(i: &[Token]) -> IResult<&[Token], char> {
    if i.len() < 1 { return IResult::Incomplete(Needed::Size(1)) }
    //println!("DEFINE {:?}", i);
    match i[0] {
        Token::Define => IResult::Done(&i[1..], '='),
        _ => IResult::Error(Err::Code(ErrorKind::Custom(6))),
    }
}

fn back_tick(i: &[Token]) -> IResult<&[Token], char> {
    if i.len() < 1 { return IResult::Incomplete(Needed::Size(1)) }
    //let (head, tail) = i.split_at(1);
    //println!("BACK_TICK {:?}", i);
    match i[0] {
        Token::BackTick => IResult::Done(&i[1..], '`'),
        _ => IResult::Error(Err::Code(ErrorKind::Custom(5))),
    }
}

fn right_paren(i: &[Token]) -> IResult<&[Token], char> {
    if i.len() < 1 { return IResult::Incomplete(Needed::Size(1)) }
    //let (head, tail) = i.split_at(1);
    //println!("RIGHT_PAREN {:?}", i);
    match i[0] {
        Token::RightParen => IResult::Done(&i[1..], ')'),
        _ => IResult::Error(Err::Code(ErrorKind::Custom(4))),
    }
}

fn left_paren(i: &[Token]) -> IResult<&[Token], char> {
    if i.len() < 1 { return IResult::Incomplete(Needed::Size(1)); }
    //let (head, tail) = i.split_at(1);
    //println!("LEFT_PAREN {:?}", i);
    match i[0] {
        Token::LeftParen => IResult::Done(&i[1..], '('),
        _ => IResult::Error(Err::Code(ErrorKind::Custom(3))),
    }
}

fn int(i: &[Token]) -> IResult<&[Token], i64> {
    if i.len() < 1 { return IResult::Incomplete(Needed::Size(1)); }
    //let (head, tail) = i.split_at(1);
    //println!("INT {:?}", i);
    match i[0] {
        Token::Int(int) => IResult::Done(&i[1..], int),
        _ => IResult::Error(Err::Code(ErrorKind::Custom(2))),
    }
}

fn identifier(i: &[Token]) -> IResult<&[Token], Identifier> {
    if i.len() < 1 { return IResult::Incomplete(Needed::Size(1)); }
    //let (head, tail) = i.split_at(1);
    //println!("IDENTIFIER {:?}", i);
    match i[0].clone() {
        Token::Identifier(id) => IResult::Done(&i[1..], Identifier::new(id)),
        _ => IResult::Error(Err::Code(ErrorKind::Custom(1))),
    }
}

fn operator(i: &[Token]) -> IResult<&[Token], Identifier> {
    if i.len() < 1 { return IResult::Incomplete(Needed::Size(1)); }
    //let (head, tail) = i.split_at(1);
    //println!("OPERATOR {:?}", i);
    match i[0].clone() {
        Token::Operator(op) => IResult::Done(&i[1..], Identifier::new(op)),
        _ => IResult::Error(Err::Code(ErrorKind::Custom(0))),
    }
}
