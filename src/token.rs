#[derive(Clone, Debug)]
pub enum Token {
    Int(i64),
    Identifier(String),
    Operator(String),
    LeftParen,
    RightParen,
    Define,
    BackTick,
    NewLine
}
