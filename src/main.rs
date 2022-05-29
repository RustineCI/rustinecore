//! The Core of RustineCI
//!
//! This crate contains all the necessary elements to create a RustineCI cli implementation.
//!
//! ! In the future it will evolve in a web based interface written here or in another crate for better readability and maintenance !
//!
//! ! The cli is **NOT** the main of the crate it's just a beggining to try the rest of the infrastructure !
//!
//! The core have for mission to manage nodes, send jobs to nodes, agglomerate the logs of every job run, store them, handle users and so much more !

#![warn(missing_docs)]

mod cli;
mod nodes;

/// The main function of the program, it currently only run the cli
/// It will later implement the configuration
fn main() {
    let mut node_list: Vec<nodes::Node> = vec![];

    println!("Welcome to the RustineCI cli !");
    loop {
        cli::cli(&mut node_list);
    }
}
