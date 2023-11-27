mod default_tree;
mod node;

use default_tree::get_default_tree;

fn main() {
    let node = get_default_tree();
    println!("{:#?}", node)
    // let _ = node.run_command();
}
