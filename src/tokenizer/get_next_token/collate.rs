use super::super::indexed_characters::IndexedCharacters;
use super::super::Token;
use crate::formatter::errors::FormatterError;

enum ComplexToken {
    False,
    True,
    Null,
}

pub fn process_expectation(
    literal: &'static str,
    token: Token,
    indexed_characters: IndexedCharacters,
) -> Result<Token, FormatterError> {
    let mut indexed_characters = indexed_characters;
    let mut character_position = indexed_characters.get_index();

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
            return Err(FormatterError::ExpectedMoreCharacters(character_position));
        };

        indexed_characters = indexed_characters.progress();
        character_position = character_position + 1;
    }

    Ok(token)
}

pub fn process_string_literal(
    indexed_characters: IndexedCharacters,
) -> Result<Token, FormatterError> {
    let position = indexed_characters.get_index();
    let mut indexed_characters = indexed_characters.progress();
    let mut character_position = position + 1;
    let mut literal = "".to_owned();
    loop {
        if let Some(&character) = indexed_characters.current_character() {
            match &character {
                '\"' => {
                    let prev_character = indexed_characters.previous_character().unwrap();
                    if *prev_character != '\\' {
                        return Ok(Token::StringLiteral(position, literal));
                    } else {
                        literal.push(character)
                    }
                }
                '\\' => {}
                value @ _ => literal.push(*value),
            }
        } else {
            return Err(FormatterError::ExpectedMoreCharacters(character_position));
        };

        indexed_characters = indexed_characters.progress();
        character_position = character_position + 1;
    }
}

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
    #[should_panic(
        expected = "Wrong Character: found (l) when expecting (s) while trying to build token false"
    )]
    fn expect_failure_when_false_is_spelt_wrong() {
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
    #[should_panic(expected = "Expected more tokens a position 3")]
    fn expect_failure_when_json_runs_out() {
        let json = "fal";
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
    fn string_literal() {
        let json = "\"tester\"";
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let expectation = Token::StringLiteral(0, String::from("tester"));
        match process_string_literal(indexed_characters) {
            Ok(result) => assert_eq!(result, expectation),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn string_literal_with_an_escape_in_it() {
        let json = r#""tes\"ter""#;
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let expectation = Token::StringLiteral(0, String::from("tes\"ter"));
        match process_string_literal(indexed_characters) {
            Ok(result) => assert_eq!(result, expectation),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn string_literal_with_lots_of_escaped_quotes_in_it() {
        let json = r#""tes\"te\"   \"r""#;
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let expectation = Token::StringLiteral(0, String::from("tes\"te\"   \"r"));
        match process_string_literal(indexed_characters) {
            Ok(result) => assert_eq!(result, expectation),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    #[should_panic(expected = "Expected more tokens a position 16")]
    fn string_literal_no_end_quote() {
        let json = r#""tes\"te\"   \"r"#;
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let token = Token::Colon(0);
        match process_string_literal(indexed_characters) {
            Ok(result) => assert_eq!(result, token),
            Err(e) => panic!("{}", e),
        }
    }
}
