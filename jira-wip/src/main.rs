#![allow(dead_code)]
use std::error::Error;

/// The `main` function is the entry point of your application.
///
/// It gets called when you invoke `cargo run --bin jira-wip` and
/// its executed when a user runs the binary you generated by compiling your project
/// (`cargo build` -> `/target/debug/jira-wip` / `cargo build --release` -> `/target/release/jira-wip`)
///
/// Over the course of this workshop we will modify this entry point to build a fully fledged
/// command line application.
///
/// Brace yourself!
fn main() -> Result<(), Box<dyn Error>> {
    // Uncomment these lines after 02_ticket_store/09_store_recap
        use path_to_enlightenment::store_recap::TicketStore;
        // Comment this line after 03_cli/01_persistence
        let mut ticket_store = TicketStore::new();

    // Uncomment these lines after 03_cli/01_persistence
    /*
        // Load the store from disk. If missing, a brand new one will be created.
        let mut ticket_store = persistence::load();
    */

    // Uncomment these lines after 03_cli/00_cli
        use path_to_enlightenment::cli::{Command, handle_command};
        // Parse the command-line arguments.
        let command = <Command as paw::ParseArgs>::parse_args()?;
        handle_command(&mut ticket_store, command)?;

    // Uncomment these lines after 03_cli/01_persistence
    /*
        // Save the store state to disk after we have completed our action.
        persistence::save(&ticket_store);
    */
    Ok(())
}

mod path_to_enlightenment;
