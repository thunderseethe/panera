use ast::function::Function;
use std::fmt;

pub struct Program {
    functions: Vec<Function>
}

impl Program {
    pub fn new(_functions: Vec<Function>) -> Self {
        Program {
            functions: _functions,
        }
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.functions)
    }
}
