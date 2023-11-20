mod node;
use std::process::Command;

use node::Node;

fn main() {
    let command = Command::new("google-chrome");
    let mut node = Node::new("Root".to_string(), "".to_string(), Some(command));
    // println!("{:#?}", node)
    let res = node.run_command();
}
