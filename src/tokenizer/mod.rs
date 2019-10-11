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
                
            println!("{} {} {:?}", start_index, end_index, token);
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

    // String
    // Number
    // Null
    // True
    // False
    // Object of various types
    // Arrays
    
    #[test]
    fn tokenize_can_consume_a_string() {
        let json = r#" { "win" : true }  "#;

        let whitespace = Token::WhiteSpace(0, ' ');
        let open_brace = Token::OpenBrace(1);
        let whitespace2 = Token::WhiteSpace(2, ' ');
        let win = Token::StringLiteral(3, String::from("win"));
        let whitespace3 = Token::WhiteSpace(8, ' ');
        let colon = Token::Colon(9);
        let whitespace4 = Token::WhiteSpace(10, ' ');
        let true_token = Token::True(11, "true");
        let whitespace5 = Token::WhiteSpace(15, ' ');
        let close_brace = Token::CloseBrace(16);
        let whitespace6 = Token::WhiteSpace(17, ' ');
        let whitespace7 = Token::WhiteSpace(18, ' ');

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
}
