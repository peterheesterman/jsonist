use super::super::indexed_characters::IndexedCharacters;
use super::super::Token;
use crate::formatter::errors::FormatterError;

pub fn check_end_for_e(token_position: usize, literal: String) -> Result<Token, FormatterError> {
    if literal.len() != 0 && literal.chars().rev().next().unwrap() == 'e' {
        Err(FormatterError::NumberLiteralEndingInE())
    } else {
        Ok(Token::Number(token_position, literal))
    }
}

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
                    return check_end_for_e(token_position, literal)
                }
                value @ _ if value.is_ascii_digit() || *value == '.' || *value == 'e' || *value == '-' => {
                    if *value == '-' && literal.len() != 0 {
                        return Err(FormatterError::NumberCanNotHaveANegativeSignNotAtHead());
                    }

                    // No second dots
                    if *value == '.' {
                        if has_seen_dot {
                            return Err(FormatterError::ExtraDotInNumber(character_position));
                        } else {
                            has_seen_dot = true;
                        }

                    // No second e
                    } else if *value == 'e' {
                        if has_seen_e {
                            return Err(FormatterError::ExtraEInNumber(character_position));
                        } else {
                            has_seen_e = true;
                        }
                    }

                    literal.push(*value)
                }
                value @ _ => {
                    return Err(FormatterError::InvalidNumberCharacter(
                        character_position,
                        *value,
                    ));
                }
            }
        } else {
            return check_end_for_e(token_position, literal);
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
    #[should_panic(
        expected = "Found and extra dot at postition (3) which is not valid in a number."
    )]
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

    #[test]
    #[should_panic(expected = "A number literal can not end with an 'e' character")]
    fn number_literal_can_not_end_with_an_e() {
        let json = r#"2.3e"#;
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let expectation = Token::Number(0, String::from(""));
        match process_number_literal(indexed_characters) {
            Ok(result) => assert_eq!(result, expectation),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn number_literals_should_handle_negative_numbers() {
        let json = r#"-2.34"#;
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let expectation = Token::Number(0, String::from("-2.34"));
        match process_number_literal(indexed_characters) {
            Ok(result) => assert_eq!(result, expectation),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    #[should_panic(expected = "Number can not have a - at a position other than the start of string")]
    fn number_literals_no_negative_symbol_at_postitions_other_than_the_first() {
        let json = r#"2-.34"#;
        let chars = json.chars().collect::<Vec<char>>();
        let indexed_characters = IndexedCharacters::new(&chars);
        let expectation = Token::Number(0, String::from("-2.34"));
        match process_number_literal(indexed_characters) {
            Ok(result) => assert_eq!(result, expectation),
            Err(e) => panic!("{}", e),
        }
    }
}
