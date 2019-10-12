
use crate::formatter::errors::FormatterError;
use crate::tokenizer::Token;

use super::Node;
use super::JumpNode;
use super::parse_node;

mod literal;
use literal::parse_literal;

pub fn parse_object(tokens: &Vec<Token>, position: usize) -> Result<JumpNode, FormatterError> {
    let mut keys: Vec<String> = vec![];
    let mut pairs = vec![];
    let mut jump = position;

    loop {
        if let Some(token) = tokens.get(jump) {
            match token {
                Token::CloseBrace(_) => return Ok((jump, Node::Object { pairs })),
                Token::Comma(_) => {
                    jump = jump + 1;
                },
                _ => {
                    let (movement, key) = parse_literal(&tokens, jump)?;
                    jump = jump + movement;

                    // Ensure there is a colon
                    if let Some(token) = tokens.get(jump) {
                        match token {
                            Token::Colon(_) => {
                                jump = jump + 1;
                            },
                            _ => return Err(FormatterError::ExpectedColonInKeyValuePair())
                        }
                    }

                    let (movement, value) = parse_node(&tokens, jump)?;
                    jump = jump + movement;

                    // Check for duplicate keys
                    if let Node::Literal { literal } = &key {
                        println!("{}", literal);
                        if keys.contains(literal) {
                            return Err(FormatterError::DuplicateKeyEntry(literal.to_string()))
                        }
                        keys.push(literal.to_string()) 
                    }


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

    #[test]
    #[should_panic(expected = "Key value pairs must be delimited by colons (:).")]
    fn parse_object_one_pair_no_colon() {
        // let json = r#" { "w in" } true }  "#;

        let open_brace = Token::OpenBrace(1);
        let win = Token::StringLiteral(3, String::from("w in"));
        let bad_char = Token::CloseBrace(10);
        let true_token = Token::True(12, "true");
        let close_brace = Token::CloseBrace(17);

        let tokens = vec![
            open_brace,
            win,
            bad_char,
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

    #[test]
    #[should_panic(expected = "Expected string literal at position (1).")]
    fn parse_object_key_must_be_a_literal() {
        // let json = r#" { false : true }  "#;

        let open_brace = Token::OpenBrace(1);
        let false_token = Token::False(12, "false");
        let colon = Token::Colon(10);
        let true_token = Token::True(12, "true");
        let close_brace = Token::CloseBrace(17);

        let tokens = vec![
            open_brace,
            false_token,
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

    #[test]
    fn parse_object_with_n_pair() {
        let open_brace = Token::OpenBrace(1);
        let win = Token::StringLiteral(3, String::from("w in"));
        let colon = Token::Colon(10);
        let true_token = Token::True(12, "true");
        let comma = Token::Comma(18);
        let win2 = Token::StringLiteral(19, String::from("wow"));
        let colon2 = Token::Colon(39);
        let false_token = Token::False(45, "false");
        let close_brace = Token::CloseBrace(56);

        let tokens = vec![
            open_brace,
            win,
            colon,
            true_token,
            comma,
            win2,
            colon2,
            false_token,
            close_brace,
        ];

        let node = Node::Object { pairs: vec![
            Box::new(Node::Pair {
                key: Box::new(Node::Literal { literal: String::from("w in") }),
                value: Box::new(Node::True)
            }),
            Box::new(Node::Pair {
                key: Box::new(Node::Literal { literal: String::from("wow") }),
                value: Box::new(Node::False)
            })
        ]};

        match parse_object(&tokens, 1) {
            Ok((_, result)) => assert_eq!(result, node),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    #[should_panic(expected = "Duplicate key ('w in') entry.")]
    fn parse_object_with_duplicate_keys_should_fail() {
        let open_brace = Token::OpenBrace(1);
        let win = Token::StringLiteral(3, String::from("w in"));
        let colon = Token::Colon(10);
        let true_token = Token::True(12, "true");
        let comma = Token::Comma(18);
        let win2 = Token::StringLiteral(19, String::from("w in"));
        let colon2 = Token::Colon(39);
        let false_token = Token::True(45, "true");
        let close_brace = Token::CloseBrace(56);

        let tokens = vec![
            open_brace,
            win,
            colon,
            true_token,
            comma,
            win2,
            colon2,
            false_token,
            close_brace,
        ];

        let node = Node::Object { pairs: vec![
            Box::new(Node::Pair {
                key: Box::new(Node::Literal { literal: String::from("w in") }),
                value: Box::new(Node::True)
            }),
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

