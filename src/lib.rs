mod tokenizer;

mod parser;

pub mod formatter;
use formatter::errors::{ FormatterError };
pub use formatter::{ FormatConfig, Delimiter, DelimiterCount };

pub fn lint(input: String, config: Option<FormatConfig>) -> Result<String, FormatterError> {
    let tokens = tokenizer::tokenize(input.as_str())?;
    let ast = parser::parse(tokens)?;
    match config {
        None => Ok(formatter::format(ast)),
        Some(config) => Ok(formatter::format_with_config(ast, &config)),
    }
}

