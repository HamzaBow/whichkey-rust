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
    let mut node1 = Node::new("Root".to_string(), "".to_string());
    node1
        .children
        .insert('a', Node::new("Child A".to_string(), "".to_string()));
    println!("{}", node1);
}
