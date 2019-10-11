use std::fmt;

use crate::formatter::errors::FormatterError;
use crate::tokenizer::Token;

mod utils;
use utils::remove_whitespace;

#[derive(Debug, PartialEq)]
struct Node {}

#[derive(Debug, PartialEq)]
pub struct AST {
    root: Node,
}

impl fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something")
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<AST, FormatterError> {
    let _tokens = remove_whitespace(tokens);

    // TODO: make the tokens into an AST

    Ok(AST { root: Node {} })
}
