mod node;
use node::Node;

fn main() {
    let node = Node::new("Root".to_string(), "".to_string(), None);
    println!("{:#?}", node)
}
