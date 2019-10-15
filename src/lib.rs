pub mod formatter;
pub use formatter::errors::{ FormatterError };
pub use formatter::{ FormatConfig, Delimiter, DelimiterCount };

mod tokenizer;
mod parser;

pub fn format(input: String, config: Option<FormatConfig>) -> Result<String, FormatterError> {
    let tokens = tokenizer::tokenize(input.as_str())?;
    let ast = parser::parse(tokens)?;
    match config {
        None => Ok(formatter::stringify(ast)),
        Some(config) => Ok(formatter::stringify_with_config(ast, &config)),
    }
}

