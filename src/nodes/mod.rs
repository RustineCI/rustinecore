//! # Node module (managements of nodes)
//!
//! This module allow to manage nodes : creation, deletion, configuration...

#![warn(missing_docs)]

/// Enum containing the differents status a Node can have
pub enum Status {
    Standby,
}

/// Struct containing all the informations necessary for a Node
///
/// # Attributes
/// name (String): The name of the Node
///
/// status (Status): The status of the Node
pub struct Node {
    name: String,
    status: Status,
}

/// Implementation of Default for the Node
///
/// It allows to create node with minimal information to avoid the need for the user to provide a name
impl std::default::Default for Node {
    fn default() -> Self {
        Node {
            name: String::from("default"),
            status: Status::Standby,
        }
    }
}

/// Implementation of Debug for the Node
///
/// It allows to print debug information about the Node
impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {}, status: {}",
            self.name,
            self.get_status_string()
        )
    }
}

/// # Implementations
impl Node {
    /// Sets the name for the Node
    ///
    /// # Arguments
    /// name (String): The name for the Node
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Gets the name of the Node
    ///
    /// # Return
    /// String: The name of the Node
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Gets the status of the Node as a String
    ///
    /// # Return
    /// String: The current status of the Node
    pub fn get_status_string(&self) -> String {
        match self.status {
            Status::Standby => String::from("STAND BY"),
        }
    }
}

/// Creates a new Node and push it in the Node list
///
/// # Arguments
/// command (std::str::SplitWhitespace): The command given by the user
///
/// node_list (Vec<Node>): The list of the current Nodes
pub fn create_node(mut command: std::str::SplitWhitespace, node_list: &mut Vec<Node>) {
    if let Some(node_name) = command.next() {
        let new_node: Node = Node {
            name: String::from(node_name),
            ..Default::default()
        };

        println!(
            "Node {} created !\n Status: {}",
            new_node.get_name(),
            new_node.get_status_string()
        );
        node_list.push(new_node);
    } else {
        let new_node: Node = Node {
            ..Default::default()
        };

        println!(
            "Node {} created !\n Status: {}",
            new_node.get_name(),
            new_node.get_status_string()
        );
        node_list.push(new_node);
    }
}

/// Prints the list the Nodes in the given list
///
/// # Arguments
/// node_list (Vec<Node>): The list of the Nodes
pub fn list_nodes(node_list: &Vec<Node>) {
    if node_list.is_empty() {
        println!("No node found, create some to display their informations when listing.");
        return;
    }
    println!("Node list:");
    for node in node_list {
        println!(
            "- Node: {}, Status: {}",
            node.get_name(),
            node.get_status_string()
        );
    }
}

/// Deletes a node in the given list
///
/// # Arguments
/// command (sd::str::SplitWhitespace): The command given by the user
///
/// node_list (Vec<Node>): The list of the Nodes
pub fn delete_node(mut command: std::str::SplitWhitespace, node_list: &mut Vec<Node>) {
    if let Some(node_name) = command.next() {
        let mut i: usize = 0;
        while i != node_list.len() {
            if node_list[i].get_name() == node_name {
                node_list.swap_remove(i);
                println!("Node {} deleted", node_name);
                break;
            }
            i += 1;
        }
    } else {
        println!("No node given for deletion");
    }
}

/// Renames a Node in the given list
///
/// # Arguments
/// command (std::str::SplitWhitespace): The command given by the user
///
/// node_list (Vec<Node>): The list of the Nodes
pub fn rename_node(mut command: std::str::SplitWhitespace, node_list: &mut Vec<Node>) {
    if let Some(node_name) = command.next() {
        if let Some(new_node_name) = command.next() {
            for node in node_list {
                if node.get_name() == node_name {
                    node.set_name(String::from(new_node_name));
                    println!("Node renamed: {}", node.get_name());
                }
            }
        }
    } else {
        println!("No node given to rename or no new name given");
    }
}

#[cfg(test)]
mod tests {
    use super::{create_node, delete_node, rename_node, Node};

    #[test]
    fn test_create_default_node() {
        let mut node_list: Vec<Node> = vec![];
        let command: std::str::SplitWhitespace = "".split_whitespace();

        create_node(command, &mut node_list);
        assert!(!node_list.is_empty());
        assert_eq!(node_list.len(), 1);
        assert_eq!(node_list.first().unwrap().name, "default");
    }

    #[test]
    fn test_create_specific_node() {
        let mut node_list: Vec<Node> = vec![];
        let command: std::str::SplitWhitespace = "toto".split_whitespace();

        create_node(command, &mut node_list);
        assert!(!node_list.is_empty());
        assert_eq!(node_list.len(), 1);
        assert_eq!(node_list.first().unwrap().name, "toto");
    }

    #[test]
    fn test_deleting_node() {
        let mut node_list: Vec<Node> = vec![];
        let command: std::str::SplitWhitespace = "toto".split_whitespace();

        create_node(command, &mut node_list);
        let command: std::str::SplitWhitespace = "toto".split_whitespace();
        delete_node(command, &mut node_list);
        assert!(node_list.is_empty());
        assert_eq!(node_list.len(), 0);
    }

    #[test]
    fn test_deleting_non_existing_node() {
        let mut node_list: Vec<Node> = vec![];
        let command: std::str::SplitWhitespace = "toto".split_whitespace();

        create_node(command, &mut node_list);
        let command: std::str::SplitWhitespace = "titi".split_whitespace();
        delete_node(command, &mut node_list);
        assert!(!node_list.is_empty());
        assert_eq!(node_list.len(), 1);
    }

    #[test]
    fn test_deleting_node_in_empty_node_list() {
        let mut node_list: Vec<Node> = vec![];
        let command: std::str::SplitWhitespace = "toto".split_whitespace();

        delete_node(command, &mut node_list);
        assert!(node_list.is_empty());
        assert_eq!(node_list.len(), 0);
    }

    #[test]
    fn test_node_renaming() {
        let mut node_list: Vec<Node> = vec![];
        let command: std::str::SplitWhitespace = "toto".split_whitespace();

        create_node(command, &mut node_list);
        let command: std::str::SplitWhitespace = "toto titi".split_whitespace();
        rename_node(command, &mut node_list);
        assert!(!node_list.is_empty());
        assert_eq!(node_list.len(), 1);
        assert_eq!(node_list.first().unwrap().name, "titi");
    }

    #[test]
    fn test_node_renaming_wrong_node_name_given() {
        let mut node_list: Vec<Node> = vec![];
        let command: std::str::SplitWhitespace = "toto".split_whitespace();

        create_node(command, &mut node_list);
        let command: std::str::SplitWhitespace = "titi titi".split_whitespace();
        rename_node(command, &mut node_list);
        assert!(!node_list.is_empty());
        assert_eq!(node_list.len(), 1);
        assert_eq!(node_list.first().unwrap().name, "toto");
    }

    #[test]
    fn test_node_renaming_no_new_name_given() {
        let mut node_list: Vec<Node> = vec![];
        let command: std::str::SplitWhitespace = "toto".split_whitespace();

        create_node(command, &mut node_list);
        let command: std::str::SplitWhitespace = "toto".split_whitespace();
        rename_node(command, &mut node_list);
        assert!(!node_list.is_empty());
        assert_eq!(node_list.len(), 1);
        assert_eq!(node_list.first().unwrap().name, "toto");
    }

    #[test]
    fn test_node_get_name() {
        let default_node: Node = Node::default();

        assert_eq!(default_node.get_name(), "default");
    }

    #[test]
    fn test_node_get_status_string() {
        let default_node: Node = Node::default();

        assert_eq!(default_node.get_status_string(), "STAND BY");
    }

    #[test]
    fn test_node_set_name() {
        let mut default_node: Node = Node::default();

        default_node.set_name(String::from("plouf"));
        assert_eq!(default_node.get_name(), "plouf");
    }
}
