use crate::formatter::errors::FormatterError;
use crate::tokenizer::Token;

use super::Node;

pub fn parse_object(tokens: Vec<Token>) -> Result<Node, FormatterError> {
    let pairs = vec![];

    // Fill up the pairs

    Ok(Node::Object { pairs })
}
