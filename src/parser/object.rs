use crate::formatter::errors::FormatterError;
use crate::tokenizer::Token;

use super::Node;
use super::JumpNode;

pub fn parse_object(tokens: &Vec<Token>, position: usize) -> Result<JumpNode, FormatterError> {
    let pairs = vec![];
    let mut jump = 0;
    jump = jump + 1;

    // Fill up the pairs

    Ok((jump, Node::Object { pairs }))
}
