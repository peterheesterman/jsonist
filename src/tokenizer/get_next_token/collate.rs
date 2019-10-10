use super::super::indexed_characters::IndexedCharacters;
use super::super::Token;
use crate::formatter::errors::FormatterError;

enum ComplexToken {
    False,
    True,
    Null
}

pub fn process_expectation(
    literal: &'static str,
    token: Token,
    indexed_characters: IndexedCharacters,
) -> Result<Token, FormatterError> {
    let mut indexed_characters = indexed_characters;

    for expected_character in literal.chars() {
        if let Some(&character) = indexed_characters.current_character() {
            if character != expected_character {
                // Wrong character error
                return Err(FormatterError::WrongCharacter {
                    attempted_token_literal: literal,
                    expected_character,
                    wrong_character: character,
                });
            }
        } else {
            return Err(FormatterError::ExpectedMoreCharacters(243243));
        };

        indexed_characters = indexed_characters.progress();
    }

    Ok(token)
}

pub fn process_string_literal(
    indexed_characters: IndexedCharacters,
) -> Result<Token, FormatterError> {
    let position = indexed_characters.get_index();
    let mut indexed_characters = indexed_characters.progress();
    let mut literal = "".to_owned();
    loop {
        if let Some(&character) = indexed_characters.current_character() {
            match &character {
                '\"' => {
                    // TODO: check previous is not an escape
                    return Ok(Token::StringLiteral(position, literal))
                },
                value @ _ => {
                    literal.push(*value)
                }
            }
        } else {
            return Err(FormatterError::ExpectedMoreCharacters(243243));
        };

        indexed_characters = indexed_characters.progress();
    }
}

// TODO: Add a method to process numbers

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_false_with_correct_tokens() {
        let json = "false";
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let token = Token::StringLiteral(0, String::from("false"));
        let token2 = Token::StringLiteral(0, String::from("false"));
        match process_expectation("false", token, indexed_characters) {
            Ok(result) => assert_eq!(result, token2),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    #[should_panic(expected = "Wrong Character: found (l) when expecting (s) while trying to build token false")]
    fn expect_failure_when_false_is_spelt_wrong () {
        let json = "fall";
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let token = Token::StringLiteral(0, String::from("false"));
        let token2 = Token::StringLiteral(0, String::from("false"));
        match process_expectation("false", token, indexed_characters) {
            Ok(result) => assert_eq!(result, token2),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn collagte_a_string_literal () {
        let json = "\"tester\"";
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let expectation = Token::StringLiteral(0, String::from("tester"));
        match process_string_literal(indexed_characters) {
            Ok(result) => assert_eq!(result, expectation),
            Err(e) => panic!("{}", e),
        }
    }
}
