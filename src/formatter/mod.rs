use super::parser::AST;
use super::parser::Node;

pub mod errors;

pub fn format(ast: AST) -> String {
    let AST { root } = ast;
    print_node(root, 0)
}

fn print_node(node: Node, depth: usize) -> String {
    // TODO: deal with passing a depth into function
    match node {
        Node::Object { pairs } => 
        {
            let depth = depth + 1;
            println!("object {}", depth);
            let padding = " ".repeat(4 * depth);
            let dedent = " ".repeat(4 * (depth - 1));
            let joiner = format!("{}{}", ",\n", padding);
            let end_object = if depth == 1 { 

                println!("happens? {}", depth);
                format!("{}{}\n", dedent, "}")
            } else {
                format!("{}{}", dedent, "}")
            };

            format!(
                "{}\n{}{}\n{}", 
                "{",
                padding,
                pairs.into_iter().map(|ref item| print_node((**item).clone(), depth)).collect::<Vec::<String>>().join(&joiner),
                &end_object
            )
        },
        Node::Array { items } => {
            let depth = depth + 1;
            println!("array {}", depth);
            let padding = " ".repeat(4 * depth);
            let dedent = " ".repeat(4 * (depth - 1));
            let joiner = format!("{}{}", ",\n", padding);
            format!(
                "[\n{}{}\n{}]", 
                padding,
                items.into_iter().map(|ref item| print_node((**item).clone(), depth)).collect::<Vec::<String>>().join(&joiner),
                dedent
            )
        },
        Node::Pair { key, value } => format!("{}: {}", print_node(*key, depth), print_node(*value, depth)),
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
            
        assert_eq!(print_node(node, 0), expected_string)
    }

    #[test]
    fn print_node_false() {
        let node = Node::False;
        let expected_string = "false";
            
        assert_eq!(print_node(node, 0), expected_string)
    }

    #[test]
    fn print_node_null() {
        let node = Node::Null;
        let expected_string = "null";
            
        assert_eq!(print_node(node, 0), expected_string)
    }

    #[test]
    fn print_node_number() {
        let node = Node::Number { value: 3.141592 };
        let expected_string = "3.141592";
            
        assert_eq!(print_node(node, 0), expected_string)
    }

    #[test]
    fn print_node_literal() {
        let node = Node::Literal { literal: "key".to_owned() };
        let expected_string = r#""key""#;
            
        assert_eq!(print_node(node, 0), expected_string)
    }

    #[test]
    fn print_node_pair() {
        let key = Node::Literal { literal: "key".to_owned() };
        let r#true = Node::True;

        let pair = Node::Pair {
            key: Box::new(key),
            value: Box::new(r#true)
        };

        let expected_string = "\"key\": true";
            
        assert_eq!(print_node(pair, 0), expected_string)
    }

    #[test]
    fn print_node_array() {
        let r#true = Node::True;
        let r#true2 = Node::True;

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
            
        assert_eq!(print_node(array, 0), expected_string)
    }

    #[test]
    fn print_node_object() {
        let key = Node::Literal { literal: "key".to_owned() };
        let r#true = Node::True;

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
            
        assert_eq!(print_node(object, 0), expected_string)
    }
}
