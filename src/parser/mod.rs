use std::fmt;

#[derive(Debug, PartialEq)]
struct Node {}

#[derive(Debug, PartialEq)]
pub struct AST {
    root: Node,
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: write something meaningful here
        write!(f, "Something")
    }
}
