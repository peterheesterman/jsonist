mod tokenizer;

mod parser;

mod formatter;
use formatter::errors::{ FormatterError };

pub fn lint(input: String) -> Result<String, FormatterError> {
    let tokens = tokenizer::tokenize(input.as_str())?;
    let ast = parser::parse(tokens)?;
    Ok(formatter::format(ast))
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: re-instate this test
    // #[test]
    // fn lint_simple_object() {
    //     let json = r#" 
    //         { 
    //           "key": true
    //         }
    //         "#.to_owned();

    //     let expected_result = "A formatted AST".to_owned();
    //     match lint(json) {
    //         Ok(result) => assert_eq!(result, expected_result),
    //         Err(e) => panic!("{}", e),
    //     }
    // }
}
