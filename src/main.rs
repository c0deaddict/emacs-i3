mod emacs;

use anyhow::Result;
use clap::{App, AppSettings, Arg};
use emacs::EmacsClient;
use i3ipc::reply::Node;
use i3ipc::I3Connection;
use std::env;

fn main() -> Result<()> {
    let matches = App::new("emacs-i3")
        .setting(AppSettings::TrailingVarArg)
        .version("0.1.2")
        .author("Jos van Bakel <jos@codeaddict.org>")
        .about("Emacs i3 integration")
        .arg(
            Arg::with_name("emacs")
                .short("e")
                .long("emacs")
                .help("Override command to send to Emacs")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("command")
                .multiple(true)
                .help("Command to send to i3 and Emacs (unless overriden)"),
        )
        .get_matches();

    let i3_command = matches
        .values_of("command")
        .unwrap_or_default()
        .collect::<Vec<_>>()
        .join(" ");

    let emacs_command = if let Some(emacs_arg) = matches.value_of("emacs") {
        emacs_arg.to_owned()
    } else {
        i3_command.clone()
    };

    let emacs_socket_path = env::var("XDG_RUNTIME_DIR").unwrap() + "/emacs/server";

    let mut i3 = I3Connection::connect().unwrap();
    let tree = i3.get_tree().unwrap();
    // TODO: find_focused can fail if the focused window is floating
    // since I rarely use Emacs in this way, revert to `to_i3=true`.
    let node = find_focused(&tree).unwrap();

    let mut to_i3 = true;

    if is_emacs(node) {
        // TODO: if eval fails, to_i3 = true
        let mut emacs = EmacsClient::new(&emacs_socket_path);
        to_i3 = emacs.eval(&emacs_i3_command(&emacs_command))? == "nil";
    }

    if to_i3 {
        let response = i3.run_command(&i3_command).unwrap();
        for outcome in response.outcomes {
            if let Some(msg) = outcome.error {
                eprintln!("i3 command '{}' failed: {}", i3_command, msg);
            }
        }
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
    if let Some(props) = node.window_properties.as_ref() {
        if let Some(class) = props.get(&i3ipc::reply::WindowProperty::Class) {
            return class == &"Emacs";
        }
    }

    node.name.as_ref().unwrap().starts_with("emacs: ")
}

/// Format the command to an expression to be run in Emacs.
fn emacs_i3_command(command: &str) -> String {
    let escaped_command = command.replace("\"", "\\\"");
    format!("(my/emacs-i3-command \"{}\")", escaped_command)
}
