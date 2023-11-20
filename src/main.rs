mod node;
use node::Node;

fn main() {
    let mut node = Node::new("Root".to_string(), "".to_string(), None);
    let child_node = Node::new("Child A".to_string(), "".to_string(), None);
    node.children.insert('a', child_node);
    node.children
        .insert('b', Node::new("Child B".to_string(), "".to_string(), None));
    node['c'] = Node::new("Child c".to_string(), "".to_string(), None);
    println!("{:#?}", node);
}

// fn print_node_if_exists(possible_node: Option<&Node>) {
//     match possible_node {
//         Some(node) => println!("{}", node),
//         None => println!("No child found"),
//     }
// }
