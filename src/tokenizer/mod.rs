use crate::formatter::errors::FormatterError;

pub mod utils;
use utils::{ get_end_index, get_start_index };

mod get_next_token;
use get_next_token::get_next_token;

pub mod indexed_characters;
use indexed_characters::IndexedCharacters;

#[derive(Debug, PartialEq)]
pub enum Token {
    // Singleton tokens
    CloseBrace(usize),
    CloseSquareBraket(usize),
    Colon(usize),
    Comma(usize),
    OpenBrace(usize),
    OpenSquareBraket(usize),
    WhiteSpace(usize, char),

    // complex tokens
    Null(usize, &'static str),
    True(usize, &'static str),
    False(usize, &'static str),
    Number(usize, String),
    StringLiteral(usize, String),
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, FormatterError> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut indexed_characters = IndexedCharacters::new(&chars);
    let mut tokens: Vec<Token>  = vec![];
 
    loop {
        if let Some(_) = indexed_characters.current_character() {
            let token = get_next_token(indexed_characters)?;

            let start_index = get_start_index(&token);
            let end_index = get_end_index(&token);
                
            indexed_characters = indexed_characters.jump(end_index - start_index);
            tokens.push(token);
        } else {
            return Ok(tokens);
        }
        indexed_characters = indexed_characters.progress();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn tokenize_string() {
        let json = r#""w in""#;
        let win = Token::StringLiteral(0, String::from("w in"));
        let tokens = vec![ win ];

        match tokenize(json) {
            Ok(result) => assert_eq!(result, tokens),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn tokenize_number() {
        let json = r#"23423.234e344"#;
        let number = Token::Number(0, String::from("23423.234e344"));
        let tokens = vec![ number ];

        match tokenize(json) {
            Ok(result) => assert_eq!(result, tokens),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn tokenize_null() {
        let json = r#"null"#;
        let null = Token::Null(0, "null");
        let tokens = vec![ null ];

        match tokenize(json) {
            Ok(result) => assert_eq!(result, tokens),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn tokenize_true() {
        let json = r#"true"#;
        let true_token = Token::True(0, "true");
        let tokens = vec![ true_token ];

        match tokenize(json) {
            Ok(result) => assert_eq!(result, tokens),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn tokenize_false() {
        let json = r#"false"#;
        let false_token = Token::False(0, "false");
        let tokens = vec![ false_token ];

        match tokenize(json) {
            Ok(result) => assert_eq!(result, tokens),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn tokenize_an_object() {
        let json = r#" { "w in" : true }  "#;

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
            whitespace, open_brace, 
                whitespace2, win, whitespace3, colon, whitespace4, true_token, whitespace5,
            close_brace, whitespace6, whitespace7
        ];

        match tokenize(json) {
            Ok(result) => assert_eq!(result, tokens),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn tokenize_an_array() {
        let json = r#" [ false, 23.23, true ]  "#;

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
            whitespace, open_bracket, whitespace2, 
                false_token, comma, whitespace3, number, comma2, whitespace4, true_token, whitespace5,
            close_bracket, whitespace6, whitespace7
        ];

        match tokenize(json) {
            Ok(result) => assert_eq!(result, tokens),
            Err(e) => panic!("{}", e),
        }
    }
}
