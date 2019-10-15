use crate::formatter::errors::FormatterError;
use crate::tokenizer::Token;

use super::JumpNode;
use super::Node;

pub fn parse_literal(tokens: &Vec<Token>, position: usize) -> Result<JumpNode, FormatterError> {
    if let Some(value) = tokens.get(position) {
        match value {
            Token::StringLiteral(_, literal) => Ok((
                1,
                (Node::Literal {
                    literal: literal.to_string(),
                }),
            )),
            _ => return Err(FormatterError::ExpectedStringLiteral(position)),
        }
    } else {
        Err(FormatterError::ExpectedMoreTokens())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_literal_normal() {
        let literal = Token::StringLiteral(0, String::from("literal literal"));

        match parse_literal(&vec![literal], 0) {
            Ok((_, result)) => assert_eq!(
                result,
                Node::Literal {
                    literal: String::from("literal literal")
                }
            ),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    #[should_panic(expected = "Expected string literal at position (0).")]
    fn parse_literal_on_other_stuff() {
        let non_literal = Token::False(0, "false");

        match parse_literal(&vec![non_literal], 0) {
            Ok((_, result)) => assert_eq!(
                result,
                Node::Literal {
                    literal: String::from("literal literal")
                }
            ),
            Err(e) => panic!("{}", e),
        }
    }
}
