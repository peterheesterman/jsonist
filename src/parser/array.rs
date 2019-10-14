use crate::formatter::errors::FormatterError;
use crate::tokenizer::Token;

use super::Node;
use super::JumpNode;
use super::parse_node;

pub fn parse_array(tokens: &Vec<Token>, position: usize) -> Result<JumpNode, FormatterError> {
    let mut items = vec![];
    let mut jump = position;

    loop {
        if let Some(token) = tokens.get(jump) {
            match token {
                Token::CloseSquareBraket(_) => {
                    let movement_from_braces = 2;
                    let net_movement = (jump - position) + movement_from_braces;
                    return Ok((net_movement, Node::Array { items }))
                },
                Token::Comma(_) => {
                    jump = jump + 1;
                },
                _ => {
                    let (movement, node) = parse_node(&tokens, jump)?;
                    jump = jump + movement;
                    items.push(Box::new(node))
                }
            }
        } else {
            return Err(FormatterError::ExpectedMoreCharacters(6666666))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_array_empty() {
        let open_bracket = Token::OpenSquareBraket(0);
        let close_bracket = Token::CloseSquareBraket(1);
        let node = Node::Array { items: vec![] };

        match parse_array(&vec![ open_bracket, close_bracket ], 1) {
            Ok((_, result)) => assert_eq!(result, node),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn parse_array_with_a_single_item() {
        let open_bracket = Token::OpenSquareBraket(0);
        let false_token = Token::False(1, "false");
        let close_bracket = Token::CloseSquareBraket(6);
        let node = Node::Array { items: vec![Box::new(Node::False)] };

        match parse_array(&vec![ open_bracket, false_token, close_bracket ], 1) {
            Ok((_, result)) => assert_eq!(result, node),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn parse_array_with_n_items() {
        let open_bracket = Token::OpenSquareBraket(0);
        let false_token = Token::False(1, "false");
        let comma = Token::Comma(6);
        let true_token = Token::True(7, "true");
        let close_bracket = Token::CloseSquareBraket(12);
        let node = Node::Array { items: vec![
            Box::new(Node::False),
            Box::new(Node::True)
        ] };

        let tokens = vec![ open_bracket, false_token, comma, true_token, close_bracket ];
        match parse_array(&tokens, 1) {
            Ok((_, result)) => assert_eq!(result, node),
            Err(e) => panic!("{}", e),
        }
    }
}
