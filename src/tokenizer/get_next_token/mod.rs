use crate::formatter::errors::FormatterError;

use super::Token;
use super::Token::*;

mod collate;
use collate::{ process_expectation, process_string_literal };
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
        ',' => Comma(position),
        '"' => process_string_literal(indexed_characters)?,
        'f' => {
            let token = Token::False(position, "false");
            process_expectation("false", token, indexed_characters)?
        },
        't' => {
            let token = Token::True(position, "true");
            process_expectation("true", token, indexed_characters)?
        },
        'n' => {
            let token = Token::Null(position, "null");
            process_expectation("null", token, indexed_characters)?
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
    fn whitespace() {
        can_create_token!(WhiteSpace(0, ' '), " ", "Can't create WhiteSpace for a space");
        can_create_token!(WhiteSpace(0, '\n'), "\n", "Can't create WhiteSpace for a newline");
        can_create_token!(WhiteSpace(0, '\t'), "\t", "Can't create WhiteSpace for a tab");
    }

    #[test]
    fn quote() {
        let chars = "\"test\"".chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);

        let token = Token::StringLiteral(0, String::from("test"));
        match get_next_token(indexed_characters) {
            Ok(value) => assert_eq!(token, value),
            Err(e) => panic!("{}", e)
        }
    }

    // #[test]
    // fn invalid_character() {
    //     // TODO: get an invalid token test in place
    //     panic!("Not an invalid token");
    // }
}
