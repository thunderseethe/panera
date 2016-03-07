use ast::value::Value;
use std::fmt;

#[derive(Clone)]
pub enum Expression {
    Application(Vec<Expression>),
    Value(Value)
}



impl Expression {
    pub fn application(_symbols: Vec<Expression>) -> Self {
        Expression::Application(_symbols)
    }

    pub fn value(_value: Value) -> Self {
        Expression::Value(_value)
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            &Expression::Application(ref params) => write!(f, "{:?}", params),
            &Expression::Value(ref val) => write!(f, "{:?}", val),
        }
    }
}
/*pub struct ApplicationExpr {
    function: Identifier,
    params: Vec<Expression>,
}

pub struct ValueExpr {
    value: Value,
}*/
