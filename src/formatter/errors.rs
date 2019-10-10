use std::fmt;
use FormatterError::*;

#[derive(Debug, PartialEq)]
pub enum FormatterError {
    ExpectedMoreCharacters(usize),
    InvalidCharacter(usize, char),
    WrongCharacter {
        attempted_token_literal: &'static str,
        expected_character: char,
        wrong_character: char,
    },
}

impl fmt::Display for FormatterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ExpectedMoreCharacters(position) => {
                write!(f, "Expected more tokens a position {}", position)
            }
            InvalidCharacter(position, character) => write!(
                f,
                "Character ({}) at postition ({}) is not valid.",
                character, 
                position
            ),
            WrongCharacter {
                attempted_token_literal,
                expected_character,
                wrong_character,
            } => write!(
                f,
                "Wrong Character: found ({}) when expecting ({}) while trying to build token {}",
                wrong_character, expected_character, attempted_token_literal
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i_can_display_an_invalid_character_display() {
        let invalid = InvalidCharacter(3, 'k');

        let description: String = format!("{}", invalid);
        assert_eq!(
            description,
            String::from("Character (k) at postition (3) is not valid.")
        )
    }
}
