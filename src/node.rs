use std::collections::HashMap;
use std::fmt::Display;
use std::io;
use std::ops::{Index, IndexMut};
use std::process::{Command, Output};

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub description: String,
    pub children: HashMap<char, Node>,
    pub command: Option<String>,
}

impl Node {
    pub fn new(name: String, description: String, command: Option<String>) -> Self {
        Self {
            name,
            description,
            children: HashMap::new(),
            command,
        }
    }

    // pub fn get_child(&self, ch: char) -> Option<&Node> {
    //     return self.children.get(&ch);
    // }

    pub fn run_command(&mut self) -> io::Result<Output> {
        match &mut self.command {
            Some(command) => Command::new("bash").arg("-c").arg(command).output(),
            None => Err(io::Error::new(io::ErrorKind::Other, "No command set")),
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

impl Index<char> for Node {
    type Output = Node;
    fn index(&self, index: char) -> &Self::Output {
        // fn index(&self, index: char) {
        self.children.get(&index).unwrap()
    }
}

impl IndexMut<char> for Node {
    fn index_mut(&mut self, index: char) -> &mut Self::Output {
        self.children.entry(index).or_insert_with(|| Node {
            name: "".to_string(),
            description: "".to_string(),
            command: None,
            children: HashMap::new(),
        })
    }
}
