use crate::formatter::errors::FormatterError;

use super::Token;
use super::Token::*;

mod collate;
use collate::*;
use super::IndexedCharacters;

pub fn get_next_token(indexed_characters: IndexedCharacters) -> Result<Token, FormatterError> {
    let position = indexed_characters.get_index();
    let character = indexed_characters.current_character().unwrap();
    let token = match character {
        '{' => OpenBrace(position),
        '}' => CloseBrace(position),
        '[' => OpenSquareBraket(position),
        ']' => CloseSquareBraket(position),
        ':' => Colon(position),
        '"' => Quote(position), // TODO: slurp up to the next quote
        'f' => {
            let literal = "false";
            let token = Token::False(position, literal);
            process_expectation(&Expectation { literal, token }, indexed_characters)?
        },
        't' => {
            let literal = "true";
            let token = Token::True(position, literal);
            process_expectation(&Expectation { literal, token }, indexed_characters)?
        },
        'n' => {
            let literal = "null";
            let token = Token::Null(position, literal);
            process_expectation(&Expectation { literal, token }, indexed_characters)?
        },
        ' ' => WhiteSpace(position, ' '),
        '\n' => WhiteSpace(position, '\n'),
        '\t' => WhiteSpace(position, '\t'),
        // TODO: add numbers
        &literal @ _ => {
            return Err(FormatterError::InvalidCharacter(position, literal))
        }
    };

    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Use macro to define the test
    macro_rules! can_create_token {
        ($expected_token: expr, $characters: expr, $error_message: expr) => {
            let chars = $characters.chars().collect::<Vec<char>>();
            let indexed_characters = IndexedCharacters::new(&chars);
            if let Ok(token) = get_next_token(indexed_characters) {
                assert_eq!(token, $expected_token);
            } else {
                panic!($error_message);
            }
        };
    }

    #[test]
    fn open_brace() {
        can_create_token!(OpenBrace(0), "{", "Can't create OpenBrace");
    }

    #[test]
    fn close_brace() {
        can_create_token!(CloseBrace(0), "}", "Can't create CloseBrace");
    }

    #[test]
    fn open_square_bracket() {
        can_create_token!(OpenSquareBraket(0), "[", "Can't create OpenBrace");
    }

    #[test]
    fn close_square_bracket() {
        can_create_token!(CloseSquareBraket(0), "]", "Can't create OpenBrace");
    }

    #[test]
    fn colon() {
        can_create_token!(Colon(0), ":", "Can't create Colon");
    }

    #[test]
    fn quote() {
        // TODO: this should slurp up a string not just 1 characteer
        can_create_token!(Quote(0), r#"""#, "Can't create Quote");
    }

    #[test]
    fn whitespace() {
        can_create_token!(WhiteSpace(0, ' '), " ", "Can't create WhiteSpace for a space");
        can_create_token!(WhiteSpace(0, '\n'), "\n", "Can't create WhiteSpace for a newline");
        can_create_token!(WhiteSpace(0, '\t'), "\t", "Can't create WhiteSpace for a tab");
    }

    #[test]
    fn invalid_character() {
        // TODO: get an invalid token test in place
        panic!("Not an invalid token");
    }
}
