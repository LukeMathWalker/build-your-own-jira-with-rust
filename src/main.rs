#![allow(clippy::new_without_default)]

use crate::models::{TicketDraft, Title};
use crate::store::TicketStore;
use std::error::Error;

pub mod models;
pub mod persistence;
pub mod store;

#[derive(structopt::StructOpt)]
/// A small command-line tool to debug connectors calls.
pub enum Command {
    /// Create a ticket on your board.
    Create {
        /// Description of the ticket.
        #[structopt(long)]
        description: String,
        /// Title of your ticket - it cannot be empty!
        #[structopt(long)]
        title: String,
    },
    Edit,
    /// Delete a ticket from the store passing the ticket id.
    Delete {
        #[structopt(long)]
        ticket_id: u64,
    },
    List,
    Move,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the command-line arguments.
    let command = <Command as paw::ParseArgs>::parse_args()?;
    // Load the store from disk. If missing, a brand new one will be created.
    let mut ticket_store = persistence::load();
    match command {
        Command::Create { description, title } => {
            let draft = TicketDraft {
                title: Title::new(title)?,
                description,
            };
            ticket_store.create(draft);
        }
        Command::Edit => todo!(),
        Command::Delete { ticket_id } => match ticket_store.delete(ticket_id) {
            Some(deleted_ticket) => println!(
                "The following ticket has been deleted:\n{:?}",
                deleted_ticket
            ),
            None => println!(
                "There was no ticket associated to the ticket id {:?}",
                ticket_id
            ),
        },
        Command::List => {
            println!("{:?}", ticket_store.list());
        }
        Command::Move => todo!(),
    }
    // Save the store state to disk after we have completed our action.
    persistence::save(&ticket_store);
    Ok(())
}
