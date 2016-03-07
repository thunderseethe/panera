use ast::identifier::Identifier;
use ast::expression::Expression;
use std::fmt;

pub struct Function {
    name: Identifier,
    args: Vec<Identifier>,
    body: Expression,
}

impl Function {
    pub fn new(_name: Identifier, _args: Vec<Identifier>, _body: Expression) -> Self{
        Function {
            name: _name,
            args: _args,
            body: _body,
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {:?} = {:?}", self.name, self.args, self.body)
    }
}
