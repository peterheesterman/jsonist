
use crate::tokenizer::Token;

pub fn remove_whitespace(tokens: Vec<Token>) -> Vec<Token> {
    tokens.into_iter().filter(|token| {
        match token {
            Token::WhiteSpace(_, _) => false,
            _ => true
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn tokenize_an_object() {
        // let json = r#" { "w in" : true }  "#;

        let whitespace = Token::WhiteSpace(0, ' ');
        let open_brace = Token::OpenBrace(1);
        let open_brace_remaining = Token::OpenBrace(1);
        let whitespace2 = Token::WhiteSpace(2, ' ');
        let win = Token::StringLiteral(3, String::from("w in"));
        let win_remaining = Token::StringLiteral(3, String::from("w in"));
        let whitespace3 = Token::WhiteSpace(9, ' ');
        let colon = Token::Colon(10);
        let colon_remaining = Token::Colon(10);
        let whitespace4 = Token::WhiteSpace(11, ' ');
        let true_token = Token::True(12, "true");
        let true_token_remaining = Token::True(12, "true");
        let whitespace5 = Token::WhiteSpace(16, ' ');
        let close_brace = Token::CloseBrace(17);
        let close_brace_remaining = Token::CloseBrace(17);
        let whitespace6 = Token::WhiteSpace(18, ' ');
        let whitespace7 = Token::WhiteSpace(19, ' ');

        let tokens = vec![
            whitespace, open_brace, 
                whitespace2, win, whitespace3, colon, whitespace4, true_token, whitespace5,
            close_brace, whitespace6, whitespace7
        ];

        let remaining_tokens = vec![ 
            open_brace_remaining,
            win_remaining,
            colon_remaining,
            true_token_remaining,
            close_brace_remaining,
        ];

        assert_eq!(remove_whitespace(tokens), remaining_tokens)
    }
}
