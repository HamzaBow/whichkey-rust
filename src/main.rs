mod node;
use node::Node;

fn main() {
    let mut node = Node::new("Root".to_string(), "".to_string());
    node.children
        .insert('a', Node::new("Child A".to_string(), "".to_string()));
    println!("{}", node);
    let child = node.get_child('a');

    match child {
        Some(child) => println!("{}", child),
        None => println!("No child found"),
    }
}
