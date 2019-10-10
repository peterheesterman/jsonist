use super::Token;
use super::Token::*;

pub fn get_start_index(token: &Token) -> usize {
    match token {
        OpenBrace(position) => *position,
        CloseBrace(position) => *position,
        OpenSquareBraket(position) => *position,
        CloseSquareBraket(position) => *position,
        Colon(position) => *position,
        Comma(position) => *position,
        WhiteSpace(position, _) => *position,

        Null(position, _) => *position,
        True(position, _) => *position,
        False(position, _) => *position,
        Number(position, _) => *position,
        StringLiteral(position, _) => *position,
    }
}

pub fn get_end_index(token: &Token) -> usize {
    match token {
        Null(position, literal) => position + literal.len() - 1,
        True(position, literal) => position + literal.len() - 1,
        False(position, literal) => position + literal.len() - 1,
        Number(position, literal) => position + literal.len() - 1,
        StringLiteral(position, literal) => position + literal.len() - 1,
        _ => get_start_index(token),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_start_position_of_simple_token() {
        let position = 3;
        let token = OpenBrace(position);
        assert_eq!(get_start_index(&token), position);
        assert_eq!(get_end_index(&token), position);
    }

    #[test]
    fn can_get_end_position_of_a_complex_token() {
        let start_position = 3;
        let token = StringLiteral(start_position, String::from("\"winning\""));
        assert_eq!(get_start_index(&token), start_position);
        assert_eq!(get_end_index(&token), 11);
    }

    #[test]
    fn can_get_end_position_of_a_complex_token_null() {
        let start_position = 3;
        let token = Null(start_position, "null");
        assert_eq!(get_start_index(&token), start_position);
        assert_eq!(get_end_index(&token), 6);
    }
}
