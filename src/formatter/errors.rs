use std::fmt;
use FormatterError::*;

#[derive(Debug, PartialEq)]
pub enum FormatterError {
    ExpectedMoreCharacters(usize),
    InvalidTokenStartCharacter(usize, char),
    WrongCharacter {
        attempted_token_literal: &'static str,
        expected_character: char,
        wrong_character: char,
    },

    // Numbers
    InvalidNumberCharacter(usize, char),
    ExtraDotInNumber(usize),
    ExtraEInNumber(usize),
}

impl fmt::Display for FormatterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ExpectedMoreCharacters(position) => {
                write!(f, "Expected more tokens a position {}", position)
            }
            InvalidTokenStartCharacter(position, character) => write!(
                f,
                "Character ({}) at postition ({}) is not valid.",
                character, 
                position
            ),
            InvalidNumberCharacter(position, character) => write!(
                f,
                "Character ({}) at postition ({}) is not valid in a number.",
                character, 
                position
            ),
            ExtraDotInNumber(position) => write!(
                f,
                "Found and extra dot at postition ({}) which is not valid in a number.",
                position
            ),
            ExtraEInNumber(position) => write!(
                f,
                "Found and extra e at postition ({}) which is not valid in a number.",
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
        let invalid = InvalidTokenStartCharacter(3, 'k');

        let description: String = format!("{}", invalid);
        assert_eq!(
            description,
            String::from("Character (k) at postition (3) is not valid.")
        )
    }
}
