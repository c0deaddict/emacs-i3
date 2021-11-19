mod emacs;

use anyhow::Result;
use emacs::EmacsClient;
use i3ipc::reply::Node;
use i3ipc::I3Connection;
use std::env;

fn main() -> Result<()> {
    let command = env::args().skip(1).collect::<Vec<_>>().join(" ");
    let emacs_socket_path = env::var("XDG_RUNTIME_DIR").unwrap() + "/emacs/server";

    let mut i3 = I3Connection::connect().unwrap();
    let tree = i3.get_tree().unwrap();
    let node = find_focused(&tree).unwrap();

    let mut to_i3 = true;
    if is_emacs(node) {
        // TODO: if eval fails, to_i3 = true
        let mut emacs = EmacsClient::new(&emacs_socket_path);
        to_i3 = emacs.eval(&emacs_i3_command(&command))? == "nil";
    }

    if to_i3 {
        i3.run_command(&command).unwrap();
    }

    Ok(())
}

/// Find the focused window in the tree.
fn find_focused<'a>(node: &'a Node) -> Option<&'a Node> {
    if node.focused {
        Some(node)
    } else {
        node.nodes.iter().find_map(find_focused)
    }
}

/// Determine if the node in question is an Emacs window.
fn is_emacs(node: &Node) -> bool {
    node.name.as_ref().unwrap().starts_with("emacs: ")
}

/// Format the command to an expression to be run in Emacs.
fn emacs_i3_command(command: &str) -> String {
    let escaped_command = command.replace("\"", "\\\"");
    format!("(my/emacs-i3-command \"{}\")", escaped_command)
}
