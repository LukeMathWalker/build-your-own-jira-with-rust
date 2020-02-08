#![allow(clippy::new_without_default)]

use crate::models::{Status, TicketDraft, TicketPatch, Title};
use std::error::Error;
use std::str::FromStr;

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
    Edit {
        #[structopt(long)]
        ticket_id: u64,
        #[structopt(long)]
        title: Option<String>,
        #[structopt(long)]
        description: Option<String>,
    },
    /// Delete a ticket from the store passing the ticket id.
    Delete {
        #[structopt(long)]
        ticket_id: u64,
    },
    List,
    Move {
        #[structopt(long)]
        ticket_id: u64,
        #[structopt(long)]
        status: Status,
    },
}

impl FromStr for Status {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        let status = match s.as_str() {
            "todo" | "to-do" => Status::ToDo,
            "inprogress" | "in-progress" => Status::InProgress,
            "blocked" => Status::Blocked,
            "done" => Status::Done,
            _ => panic!("The status you specified is not valid. Valid values: todo, inprogress, blocked and done.")
        };
        Ok(status)
    }
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
        Command::Edit {
            ticket_id,
            title,
            description,
        } => {
            let title = title.map(Title::new).transpose()?;
            let ticket_patch = TicketPatch { title, description };
            match ticket_store.update_ticket(ticket_id, ticket_patch) {
                Some(_) => println!("Ticket {:?} was updated.", ticket_id),
                None => println!(
                    "There was no ticket associated to the ticket id {:?}",
                    ticket_id
                ),
            }
        }
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
            let ticket_list = ticket_store.list().into_iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join("\n\n");
            println!("{}", ticket_list);
        }
        Command::Move { ticket_id, status } => {
            match ticket_store.update_ticket_status(ticket_id, status) {
                Some(_) => println!(
                    "Status of ticket {:?} was updated to {:?}",
                    ticket_id, status
                ),
                None => println!(
                    "There was no ticket associated to the ticket id {:?}",
                    ticket_id
                ),
            }
        }
    }
    // Save the store state to disk after we have completed our action.
    persistence::save(&ticket_store);
    Ok(())
}
