use std::fmt;

use crate::formatter::errors::FormatterError;
use crate::tokenizer::Token;

mod utils;
use utils::remove_whitespace;

mod object;
use object::parse_object;

mod array;
use array::parse_array;

#[derive(Debug, PartialEq)]
pub enum Node {
    Object { pairs: Vec<Box<Node>> },
    Array { items: Vec<Box<Node>> },
    Pair { key: Box<Node>, value: Box<Node> },
    Literal { literal: String },
    Number { value: f64 },
    True,
    False,
    Null,
}

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
    let tokens = remove_whitespace(tokens);
    let node = parse_node(tokens, 0)?;

    Ok(AST { root: node })
}

fn parse_node(tokens: Vec<Token>, position: usize) -> Result<Node, FormatterError> {
    if let Some(value) = tokens.get(position) {
        match value {
            Token::OpenBrace(_) => Ok(parse_object(tokens, position + 1)?),
            Token::OpenSquareBraket(_) => Ok(parse_array(tokens, position + 1)?),
            Token::True(_, _) => Ok(Node::True),
            Token::False(_, _) => Ok(Node::False),
            Token::Null(_, _) => Ok(Node::Null),
            Token::StringLiteral(_, literal) => Ok(Node::Literal { literal: literal.to_string() }),
            Token::Number(_, literal) => Ok(Node::Number { value:  literal.parse::<f64>().unwrap() }), // unwrap?
            _ => return Err(FormatterError::ExpectedMoreCharacters(11111111))
        }
    } else {
        Ok(Node::False)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn r#true() {
        let true_token = Token::True(0, "true");
        let ast = AST { root: Node::True };

        match parse(vec![ true_token ]) {
            Ok(result) => assert_eq!(result, ast),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn r#false() {
        let false_token = Token::False(0, "false");
        let ast = AST { root: Node::False };

        match parse(vec![ false_token ]) {
            Ok(result) => assert_eq!(result, ast),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn null() {
        let null = Token::Null(0, "null");
        let ast = AST { root: Node::Null };

        match parse(vec![ null ]) {
            Ok(result) => assert_eq!(result, ast),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn string() {
        let input = String::from("test mc test");
        let string = Token::StringLiteral(0, input.to_string());
        let ast = AST { root: Node::Literal { literal: input } };

        match parse(vec![ string ]) {
            Ok(result) => assert_eq!(result, ast),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn number() {
        let number = Token::Number(0, String::from("34.4e3"));
        let ast = AST { root: Node::Number { value: 34400.0 } };

        match parse(vec![ number ]) {
            Ok(result) => assert_eq!(result, ast),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn array() {
        // let json = r#" [ false, 23.23, true ]  "#;

        let whitespace = Token::WhiteSpace(0, ' ');
        let open_bracket = Token::OpenSquareBraket(1);
        let whitespace2 = Token::WhiteSpace(2, ' ');
        let false_token = Token::False(3, "false");
        let comma = Token::Comma(8);
        let whitespace3 = Token::WhiteSpace(9, ' ');
        let number = Token::Number(10, String::from("23.23"));
        let comma2 = Token::Comma(15);

        let whitespace4 = Token::WhiteSpace(16, ' ');

        let true_token = Token::True(17, "true");
        let whitespace5 = Token::WhiteSpace(21, ' ');
        let close_bracket = Token::CloseSquareBraket(22);
        let whitespace6 = Token::WhiteSpace(23, ' ');
        let whitespace7 = Token::WhiteSpace(24, ' ');

        let tokens = vec![
            whitespace,
            open_bracket,
            whitespace2,
            false_token,
            comma,
            whitespace3,
            number,
            comma2,
            whitespace4,
            true_token,
            whitespace5,
            close_bracket,
            whitespace6,
            whitespace7,
        ];

        let ast = AST { root: Node::Array { items: vec![
            Box::new(Node::False)
        ] } };

        match parse(tokens) {
            Ok(result) => assert_eq!(result, ast),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn object() {
        // let json = r#" { "w in" : true }  "#;

        let whitespace = Token::WhiteSpace(0, ' ');
        let open_brace = Token::OpenBrace(1);
        let whitespace2 = Token::WhiteSpace(2, ' ');
        let win = Token::StringLiteral(3, String::from("w in"));
        let whitespace3 = Token::WhiteSpace(9, ' ');
        let colon = Token::Colon(10);
        let whitespace4 = Token::WhiteSpace(11, ' ');
        let true_token = Token::True(12, "true");
        let whitespace5 = Token::WhiteSpace(16, ' ');
        let close_brace = Token::CloseBrace(17);
        let whitespace6 = Token::WhiteSpace(18, ' ');
        let whitespace7 = Token::WhiteSpace(19, ' ');

        let tokens = vec![
            whitespace,
            open_brace,
            whitespace2,
            win,
            whitespace3,
            colon,
            whitespace4,
            true_token,
            whitespace5,
            close_brace,
            whitespace6,
            whitespace7,
        ];

        let ast = AST { root: Node::Object { pairs: vec![] } };

        match parse(tokens) {
            Ok(result) => assert_eq!(result, ast),
            Err(e) => panic!("{}", e),
        }
    }
}
