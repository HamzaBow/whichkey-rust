use std::fmt::Display;
struct Node {
    name: String,
    description: String,
}

impl Node {
    fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, description: {}", self.name, self.description)
    }
}

fn main() {
    // TODO: figure out how to make the description argument optional
    let node1 = Node::new("Root".to_string(), "".to_string());
    // let node2 = Node::from(node1);

    println!("node1: -> {}", node1);
    // println!("node2: {}", node2);
}
