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
/// node_list (Vec<Node>): The list of the Nodes
pub fn delete_node(mut command: std::str::SplitWhitespace, node_list: &mut Vec<Node>) {
    if let Some(node_name) = command.next() {
        let mut i: usize = 0;
        while i != node_list.len() {
            if node_list[i].get_name() == node_name {
                break;
            }
            i += 1;
        }
        node_list.swap_remove(i);
    } else {
        println!("No node given for deletion");
    }
}

/// Renames a Node in the given list
///
/// # Arguments
/// command (std::str::SplitWhitespace): The command given by the user
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
