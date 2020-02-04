#![allow(clippy::new_without_default)]

use crate::models::{TicketDraft, Title};
use crate::store::TicketStore;
use std::error::Error;

pub mod models;
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
    Delete,
    List,
    Move,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse the command-line arguments.
    let command = <Command as paw::ParseArgs>::parse_args()?;
    let mut ticket_store = TicketStore::new();
    match command {
        Command::Create { description, title } => {
            let draft = TicketDraft {
                title: Title::new(title)?,
                description,
            };
            ticket_store.create(draft);
            println!("{:?}", ticket_store.list());
        }
        Command::Edit => todo!(),
        Command::Delete => todo!(),
        Command::List => todo!(),
        Command::Move => todo!(),
    }
    Ok(())
}
