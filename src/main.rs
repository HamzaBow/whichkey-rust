mod node;
use node::Node;

fn main() {
    let mut node = Node::new("Root".to_string(), "".to_string());
    let child_node = Node::new("Child A".to_string(), "".to_string());
    node.children.insert('a', child_node);
    node.children
        .insert('b', Node::new("Child B".to_string(), "".to_string()));
    println!("{}", node);
    let child_a = node.get_child('a');
    let child_b = node.get_child('b');

    print_node_if_exists(child_a);
    print_node_if_exists(child_b);
}

fn print_node_if_exists(possible_node: Option<&Node>) {
    match possible_node {
        Some(node) => println!("{}", node),
        None => println!("No child found"),
    }
}
