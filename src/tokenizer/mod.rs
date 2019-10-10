use crate::formatter::errors::FormatterError;

pub mod utils;

mod get_next_token;
use get_next_token::get_next_token;

pub mod indexed_characters;
use indexed_characters::IndexedCharacters;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    // Singleton tokens
    CloseBrace(usize),
    CloseSquareBraket(usize),
    Colon(usize),
    Comma(usize),
    OpenBrace(usize),
    OpenSquareBraket(usize),
    Quote(usize),
    WhiteSpace(usize, char),

    // complex tokens
    Null(usize, &'static str),
    True(usize, &'static str),
    False(usize, &'static str),
    Number(usize, &'static str),
    StringLiteral(usize, &'static str),
}

// TODO: write this with a fold one day
pub fn tokenize(input: &str) -> Result<Vec<Token>, FormatterError> {
    let chars = input.chars().collect::<Vec<char>>();
    let mut indexed_characters = IndexedCharacters::new(&chars);
    let mut tokens: Vec<Token>  = vec![];
 
    loop {
        if let Some(character) = indexed_characters.current_character() {
            let token = get_next_token(indexed_characters)?;
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
    fn tokenize_can_consume_a_string() {
        // TODO: expand this test
        assert_eq!(true, true);
    }
}
