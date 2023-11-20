use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Index;
use std::process::Command;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub description: String,
    pub children: HashMap<char, Node>,
    pub command: Option<Command>,
}

impl Node {
    pub fn new(name: String, description: String, command: Option<Command>) -> Self {
        Self {
            name,
            description,
            children: HashMap::new(),
            command,
        }
    }

    pub fn get_child(&self, ch: char) -> Option<&Node> {
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

impl Index<char> for Node {
    type Output = Node;
    fn index(&self, index: char) -> &Self::Output {
        // fn index(&self, index: char) {
        self.children.get(&index).unwrap()
    }
}
