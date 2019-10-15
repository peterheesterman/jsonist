use super::parser::AST;
use super::parser::Node;

pub mod errors;

pub fn format(ast: AST) -> String {
    let AST { root } = ast;
    let config = FormatConfig { delimiter: Delimiter::Spaces(DelimiterCount::Four) };
    print_node(root, 0, &config)
}

pub fn format_with_config(ast: AST, config: &FormatConfig) -> String {
    let AST { root } = ast;
    print_node(root, 0, config)
}

pub enum DelimiterCount { Four, Two }
pub enum Delimiter { Spaces(DelimiterCount), Tabs }
pub struct FormatConfig { delimiter: Delimiter }

impl FormatConfig {
    pub fn new(delimiter: Delimiter) -> FormatConfig {
        FormatConfig { delimiter }
    }
}

fn derive(depth: usize, config: &FormatConfig) -> (String, String) {
    match &config.delimiter {
        Delimiter::Spaces(count) => {
            let number = match count {
                DelimiterCount::Two => 2,
                DelimiterCount::Four => 4,
            };
            (" ".repeat(number * (depth + 1)), " ".repeat(number * depth))
        },
        Delimiter::Tabs => ("\t".repeat(depth + 1), "\t".repeat(depth))
    }
}

fn print_node(node: Node, depth: usize, config: &FormatConfig) -> String {
    match node {
        Node::Object { pairs } => {
            let (indent, dedent) = derive(depth, config);
            let end = format!("{}{}{}", dedent, "}", if depth == 0 { "\n" } else { "" });
            let joiner = format!("{}{}", ",\n", indent);

            format!(
                "{}\n{}{}\n{}", 
                "{",
                indent,
                pairs.into_iter().map(|ref item| print_node((**item).clone(), depth + 1, config)).collect::<Vec::<String>>().join(&joiner),
                &end
            )
        },
        Node::Array { items } => {
            let (indent, dedent) = derive(depth, config);
            let joiner = format!("{}{}", ",\n", indent);
            format!(
                "[\n{}{}\n{}]", 
                indent,
                items.into_iter().map(|ref item| print_node((**item).clone(), depth + 1, config)).collect::<Vec::<String>>().join(&joiner),
                dedent
            )
        },
        Node::Pair { key, value } => format!("{}: {}", print_node(*key, depth, config), print_node(*value, depth, config)),
        Node::Literal { literal } => format!("\"{}\"", literal),
        Node::Number { value } => format!("{}", value),
        Node::True => String::from("true"),
        Node::False => String::from("false"),
        Node::Null => String::from("null"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_node_true() {
        let node = Node::True;
        let expected_string = "true";
        let config = FormatConfig { delimiter: Delimiter::Spaces(DelimiterCount::Four) };
            
        assert_eq!(print_node(node, 0, &config), expected_string)
    }

    #[test]
    fn print_node_false() {
        let node = Node::False;
        let expected_string = "false";
        let config = FormatConfig { delimiter: Delimiter::Spaces(DelimiterCount::Four) };
            
        assert_eq!(print_node(node, 0, &config), expected_string)
    }

    #[test]
    fn print_node_null() {
        let node = Node::Null;
        let expected_string = "null";
        let config = FormatConfig { delimiter: Delimiter::Spaces(DelimiterCount::Four) };
            
        assert_eq!(print_node(node, 0, &config), expected_string)
    }

    #[test]
    fn print_node_number() {
        let node = Node::Number { value: 3.141592 };
        let expected_string = "3.141592";
        let config = FormatConfig { delimiter: Delimiter::Spaces(DelimiterCount::Four) };
            
        assert_eq!(print_node(node, 0, &config), expected_string)
    }

    #[test]
    fn print_node_literal() {
        let node = Node::Literal { literal: "key".to_owned() };
        let expected_string = r#""key""#;
        let config = FormatConfig { delimiter: Delimiter::Spaces(DelimiterCount::Four) };
            
        assert_eq!(print_node(node, 0, &config), expected_string)
    }

    #[test]
    fn print_node_pair() {
        let key = Node::Literal { literal: "key".to_owned() };
        let r#true = Node::True;
        let config = FormatConfig { delimiter: Delimiter::Spaces(DelimiterCount::Four) };

        let pair = Node::Pair {
            key: Box::new(key),
            value: Box::new(r#true)
        };

        let expected_string = "\"key\": true";
            
        assert_eq!(print_node(pair, 0, &config), expected_string)
    }

    #[test]
    fn print_node_array() {
        let r#true = Node::True;
        let r#true2 = Node::True;
        let config = FormatConfig { delimiter: Delimiter::Spaces(DelimiterCount::Four) };

        let array = Node::Array {
            items: vec![
                Box::new(r#true),
                Box::new(r#true2),
            ]
        };

        let expected_string = "[
    true,
    true
]";
            
        assert_eq!(print_node(array, 0, &config), expected_string)
    }

    #[test]
    fn print_node_object() {
        let key = Node::Literal { literal: "key".to_owned() };
        let r#true = Node::True;
        let config = FormatConfig { delimiter: Delimiter::Spaces(DelimiterCount::Four) };

        let pair = Node::Pair {
            key: Box::new(key),
            value: Box::new(r#true)
        };
        
        let object = Node::Object {
            pairs: vec![
                Box::new(pair)
            ]
        };

        let expected_string = r#"{
    "key": true
}
"#;
            
        assert_eq!(print_node(object, 0, &config), expected_string)
    }
}
