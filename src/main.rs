mod node;
use std::process::Command;

use node::Node;

fn main() {
    let command = Command::new("microsoft-edge");
    let node = Node::new("Root".to_string(), "".to_string(), Some(command));
    // println!("{:#?}", node)
    let res = node.command.unwrap().output();
}
