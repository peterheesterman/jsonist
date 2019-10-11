use crate::formatter::errors::FormatterError;
use crate::tokenizer::Token;

use super::Node;

pub fn parse_array(tokens: Vec<Token>) -> Result<Node, FormatterError> {
    let items = vec![];

    // Fill up the pairs

    Ok(Node::Array { items })
}
