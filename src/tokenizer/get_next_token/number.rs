use super::super::indexed_characters::IndexedCharacters;
use super::super::Token;
use crate::formatter::errors::FormatterError;

pub fn process_number_literal(
    indexed_characters: IndexedCharacters,
) -> Result<Token, FormatterError> {
    let token_position = indexed_characters.get_index();
    let mut indexed_characters = indexed_characters;
    let mut character_position = indexed_characters.get_index();
    let mut literal = "".to_owned();
    let mut has_seen_dot = false;
    let mut has_seen_e = false;
    loop {
        if let Some(&character) = indexed_characters.current_character() {
            match &character {
                ',' | ']' | '}' | ' ' | '\n' | '\t' => {
                    return Ok(Token::Number(token_position, literal))
                },
                value @ _ if value.is_ascii_digit() || *value == '.' || *value == 'e' => {
                    // dis-allow second dots?
                    if *value == '.' {
                        if has_seen_dot {
                            return Err(FormatterError::ExtraDotInNumber(character_position));
                        } else {
                            has_seen_dot = true;
                        }
                    } else if *value == 'e' {
                        if has_seen_e {
                            return Err(FormatterError::ExtraEInNumber(character_position));
                        } else {
                            has_seen_e = true;
                        }
                    } 

                    literal.push(*value)
                },
                value @ _ => {
                    return Err(FormatterError::InvalidNumberCharacter(character_position, *value));
                }
            }
        } else {
            return Ok(Token::Number(token_position, literal))
        };

        indexed_characters = indexed_characters.progress();
        character_position = character_position + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_literal() {
        let json = r#"2.34e3"#;
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let expectation = Token::Number(0, String::from("2.34e3"));
        match process_number_literal(indexed_characters) {
            Ok(result) => assert_eq!(result, expectation),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    #[should_panic(expected = "Character (f) at postition (4) is not valid in a number.")]
    fn number_can_not_contain_letters_other_than_e() {
        let json = r#"2324f"#;
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let expectation = Token::Number(0, String::from(""));
        match process_number_literal(indexed_characters) {
            Ok(result) => assert_eq!(result, expectation),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    #[should_panic(expected = "Found and extra dot at postition (3) which is not valid in a number.")]
    fn number_literal_can_not_have_two_dots() {
        let json = r#"2.3.4e3"#;
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let expectation = Token::Number(0, String::from(""));
        match process_number_literal(indexed_characters) {
            Ok(result) => assert_eq!(result, expectation),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    #[should_panic(expected = "Found and extra e at postition (5) which is not valid in a number.")]
    fn number_literal_can_not_have_two_exponentials() {
        let json = r#"2.3e4e3"#;
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let expectation = Token::Number(0, String::from(""));
        match process_number_literal(indexed_characters) {
            Ok(result) => assert_eq!(result, expectation),
            Err(e) => panic!("{}", e),
        }
    }
}
