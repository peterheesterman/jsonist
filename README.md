# Jsonist

A JSON formatter


### How does it work?

Jsonist tokenizes the input string and then builds an Abstract Syntax Tree (AST).

It then takes the AST and produces a String of formatted JSON from the AST and optional configuration variables.


### Example Usage

Add to your `Cargo.toml`:

```
   … Fill this in
```

Then in your code:

```
extern crate jsonist;
use jsonist:: { format, FormatConfig, Delimiter, DelimiterCount, FormatterError };

fn example() {
    let json = r#"
        {
            "name": "Peter",
            "leg_count": 2,
            "languages": ["rust", "javascript", "lisp"],
            "address": {
                "street_name": "lets not put this online",
                "city": "a large one"
            },
            "winner": true
        }
    "#.to_owned();

    // let config = FormatConfig::new(Delimiter::Tabs);
    let config = FormatConfig::new(Delimiter::Spaces(DelimiterCount::Two));

    match format(json, config) {
        Ok(formatted_json) => {
            … do what you want with the 'formatted_json'
        }
        Err(e) => panic!("{}", e)
    }
}
```


### Error types
(in case you want to handle, ignore or print them out)
``` 
    // Tokeniser
    ExpectedMoreCharacters, InvalidTokenStartCharacter, WrongCharacter

      // Tokenising Numbers 
      InvalidNumberCharacter, ExtraDotInNumber, ExtraEInNumber, NumberLiteralEndingInE,

    // Parser
    ExpectedMoreTokens, ExpectedColonInKeyValuePair, ExpectedStringLiteral, DuplicateKeyEntry
```
