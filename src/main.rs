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
        write!(
            f,
            "Node(name = \"{}\", description = \"{}\")",
            self.name, self.description
        )
    }
}

fn main() {
    let node1 = Node::new("Root".to_string(), "".to_string());

    println!("{}", node1);
}
