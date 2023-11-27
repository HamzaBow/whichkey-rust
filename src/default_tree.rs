use crate::node::Node;

pub fn get_default_tree() -> Node {
    return Node::new(
        "Root".to_string(),
        "".to_string(),
        Some(String::from(
            "google-chrome --new-window --profile-directory=\"Profile 5\"",
        )),
    );
}
