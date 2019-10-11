use crate::formatter::errors::FormatterError;
use crate::tokenizer::Token;

use super::Node;
use super::JumpNode;
use super::parse_node;

pub fn parse_object(tokens: &Vec<Token>, position: usize) -> Result<JumpNode, FormatterError> {
    let mut pairs = vec![];
    let mut jump = position;

    loop {
        if let Some(token) = tokens.get(jump) {
                println!("{:?}", token);
            match token {
                Token::CloseBrace(_) => return Ok((jump, Node::Object { pairs })),
                Token::Comma(_) => {
                    jump = jump + 1;
                },
                _ => {
                    let (movement, key) = parse_node(&tokens, jump)?;
                    jump = jump + movement + 1;

                    // TODO: check for a colon
                    let (movement, value) = parse_node(&tokens, jump)?;
                    jump = jump + movement;

                    pairs.push(Box::new(Node::Pair { key: Box::new(key), value: Box::new(value) }))
                }
            }
        } else {
            return Err(FormatterError::ExpectedMoreCharacters(2))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::AST;
    use super::*;

    #[test]
    fn parse_object_empty() {
        let open_brace = Token::OpenBrace(0);
        let close_brace = Token::CloseBrace(1);
        let node = Node::Object { pairs: vec![] };

        match parse_object(&vec![ open_brace, close_brace ], 1) {
            Ok((_, result)) => assert_eq!(result, node),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn parse_object_one_pair() {
        // let json = r#" { "w in" : true }  "#;

        let open_brace = Token::OpenBrace(1);
        let win = Token::StringLiteral(3, String::from("w in"));
        let colon = Token::Colon(10);
        let true_token = Token::True(12, "true");
        let close_brace = Token::CloseBrace(17);

        let tokens = vec![
            open_brace,
            win,
            colon,
            true_token,
            close_brace,
        ];

        let node = Node::Object { pairs: vec![
            Box::new(Node::Pair {
                key: Box::new(Node::Literal { literal: String::from("w in") }),
                value: Box::new(Node::True)
            })
        ]};

        match parse_object(&tokens, 1) {
            Ok((_, result)) => assert_eq!(result, node),
            Err(e) => panic!("{}", e),
        }
    }
}

