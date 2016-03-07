use ast::identifier::Identifier;
use std::fmt;

#[derive(Clone)]
pub enum Value {
    Int(i64),
    Variable(Identifier),
}

impl Value {
    pub fn int(_i: i64) -> Self {
        Value::Int(_i)
    }

    pub fn variable(_var: Identifier) -> Self {
        Value::Variable(_var)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            &Value::Int(i) => write!(f, "{}", i),
            &Value::Variable(ref var) => write!(f, "{:?}", var),
        }
    }
}
