mod node;

use node::Node;

fn main() {
    let mut node = Node::new(
        "Root".to_string(),
        "".to_string(),
        Some(String::from(
            "google-chrome --new-window --profile-directory=\"Profile 5\"",
        )),
    );
    // println!("{:#?}", node)
    let _ = node.run_command();
}
