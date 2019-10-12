use super::parser::AST;
use super::parser::Node;

pub mod errors;

pub fn format(ast: AST) -> String {
    let AST { root } = ast;
    print_node(root)
}

fn print_node(node: Node) -> String {
    // TODO: deal with passing a depth into function
    match node {
        //TODO: implement and test object nodes
        // Node::Object { pairs } => 
        Node::Array { items } => format!(
            "[\n\t{}\n]", 
            items.into_iter().map(|ref item| print_node((**item).clone())).collect::<Vec::<String>>().join(",\n\t")
        ),
        Node::Pair { key, value } => format!("{}: {}", print_node(*key), print_node(*value)),
        Node::Literal { literal } => format!("{}", literal),
        Node::Number { value } => format!("{}", value),
        Node::True => String::from("true"),
        Node::False => String::from("false"),
        Node::Null => String::from("null"),
        _ => panic!("failed")
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_node_true() {
        let node = Node::True;
        let expected_string = "true";
            
        assert_eq!(print_node(node), expected_string)
    }

    #[test]
    fn print_node_false() {
        let node = Node::False;
        let expected_string = "false";
            
        assert_eq!(print_node(node), expected_string)
    }

    #[test]
    fn print_node_null() {
        let node = Node::Null;
        let expected_string = "null";
            
        assert_eq!(print_node(node), expected_string)
    }

    #[test]
    fn print_node_number() {
        let node = Node::Number { value: 3.141592 };
        let expected_string = "3.141592";
            
        assert_eq!(print_node(node), expected_string)
    }

    #[test]
    fn print_node_literal() {
        let node = Node::Literal { literal: r#""key""#.to_owned() };
        let expected_string = r#""key""#;
            
        assert_eq!(print_node(node), expected_string)
    }

    #[test]
    fn print_node_pair() {
        let key = Node::Literal { literal: r#""key""#.to_owned() };
        let r#true = Node::True;

        let pair = Node::Pair {
            key: Box::new(key),
            value: Box::new(r#true)
        };

        let expected_string = r#""key": true"#;
            
        assert_eq!(print_node(pair), expected_string)
    }

    #[test]
    fn print_node_array() {
        let key = Node::Literal { literal: r#""key""#.to_owned() };
        let r#true = Node::True;
        let r#true2 = Node::True;

        let array = Node::Array {
            items: vec![
                Box::new(r#true),
                Box::new(r#true2),
            ]
        };

        let expected_string = "[
\ttrue,
\ttrue
]";
            
        assert_eq!(print_node(array), expected_string)
    }
}
