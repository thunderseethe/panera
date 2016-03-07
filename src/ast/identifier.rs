use std::fmt;

#[derive(Clone)]
pub struct Identifier {
    name: String
}

impl Identifier {
    pub fn new(_name: String) -> Self {
        Identifier {
            name: _name
        }
    }
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}
