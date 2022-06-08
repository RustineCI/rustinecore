//! # Cli (command-line)
//!
//! This module allow to handle cli as a first step for the basic tests before the web interface do the job

#![warn(missing_docs)]

use crate::nodes;

/// Prints a prompt, take user input and give the result to the match_command function
pub fn cli(node_list: &mut Vec<nodes::Node>) {
    let mut command: String = String::new();

    print!("> ");
    std::io::Write::flush(&mut std::io::stdout()).expect("Enable to flush stdout");
    std::io::stdin()
        .read_line(&mut command)
        .expect("Did not enter a correct string");
    command.pop();
    match_command(command, node_list);
}

/// Check wich command is typed to call the right function
///
/// If the function is not known it prints "Command not found"
///
/// # Arguments
/// Takes a String as parameter, it will be used to match with the available commands.
///
/// # Available commands
/// exit: to exit the program
/// node: to access the node possibilities
fn match_command(command: String, node_list: &mut Vec<nodes::Node>) {
    let mut command: std::str::SplitWhitespace = command.split_whitespace();

    match command.next() {
        Some("exit") => exit(),
        Some("node") => node(command, node_list),
        _ => println!("Wrong command !"),
    }
}

/// This function exit the process
///
/// It should be called **only** with match_command to avoid errors or misunderstanding
fn exit() {
    println!("Bye friend !");
    std::process::exit(0);
}

/// Execute node commands
///
/// If the command is unknown it prints "Not a node command" and return
///
/// # Arguments
/// Takes a std::str::SplitWhitespace as parameter, it will be used to match with the differents node commands
fn node(mut command: std::str::SplitWhitespace, node_list: &mut Vec<nodes::Node>) {
    match command.next() {
        Some("create") => nodes::create_node(command, node_list),
        Some("list") => nodes::list_nodes(node_list),
        Some("delete") => nodes::delete_node(command, node_list),
        Some("rename") => nodes::rename_node(command, node_list),
        _ => println!("Not a node command"),
    }
}
