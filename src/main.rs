use std::collections::HashMap;
use std::fmt::Display;

struct Node {
    name: String,
    description: String,
    children: HashMap<char, Node>,
}

impl Node {
    fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            children: HashMap::new(),
        }
    }

    fn get_child(&self, ch: char) -> Option<&Node> {
        return self.children.get(&ch);
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node(name = \"{}\", description = \"{}\", number of children = {})",
            self.name,
            self.description,
            self.children.len()
        )
    }
}

fn main() {
    let mut node = Node::new("Root".to_string(), "".to_string());
    node.children
        .insert('a', Node::new("Child A".to_string(), "".to_string()));
    println!("{}", node);
    // let child = node.children.get(&'a');
    let child = node.get_child('a');

    match child {
        Some(child) => println!("{}", child),
        None => println!("No child found"),
    }
}
