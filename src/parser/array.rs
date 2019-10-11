use crate::formatter::errors::FormatterError;
use crate::tokenizer::Token;

use super::Node;
use super::parse_node;

pub fn parse_array(tokens: Vec<Token>, position: usize) -> Result<Node, FormatterError> {
    let mut items = vec![];

    if let Some(token) = tokens.get(position) {
        println!("{:?}", token);
        match token {
            Token::CloseSquareBraket(_) => return Ok(Node::Array { items }),
            _ => {
                let node = parse_node(tokens, position)?;
                items.push(Box::new(node))
            }
        }
    } else {
        return Err(FormatterError::ExpectedMoreCharacters(2))
    }

    return Ok(Node::Array { items })
}

#[cfg(test)]
mod tests {
    use super::super::AST;
    use super::*;

    #[test]
    fn parse_array_empty() {
        let open_bracket = Token::OpenSquareBraket(0);
        let close_bracket = Token::CloseSquareBraket(1);
        let node = Node::Array { items: vec![] };

        match parse_array(vec![ open_bracket, close_bracket ], 1) {
            Ok(result) => assert_eq!(result, node),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn parse_array_with_a_single_item() {
        let open_bracket = Token::OpenSquareBraket(0);
        let false_token = Token::False(1, "false");
        let close_bracket = Token::CloseSquareBraket(6);
        let node = Node::Array { items: vec![Box::new(Node::False)] };

        match parse_array(vec![ open_bracket, false_token, close_bracket ], 1) {
            Ok(result) => assert_eq!(result, node),
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
        match parse_array(tokens, 1) {
            Ok(result) => assert_eq!(result, node),
            Err(e) => panic!("{}", e),
        }
    }
}
