/// There are many ways to expose the functionality we built to a user: an API, a GUI, etc.
/// We will go with something simpler, yet good enough to probe at our implementation
/// and touch with our own hands the fruit of our labor: a command line application, a CLI.
///
/// Rust is well-equipped to write CLIs: we will be using `structopt`, a crate
/// that provides a derive macro to define a CLI interface declaratively.
///
/// We define the structure of our commands, annotating each field appropriately,
/// and `#[derive(structopt::StructOpt)]` takes care of generating all the code
/// required to parse the user input as well as generating a detailed `--help` page
/// for the CLI itself and each of its subcommands.
///
/// Comments on each of the field and each of the `Command` variant will be shown in the
/// help page of those commands!
///
/// You can learn more about `structopt` looking at their documentation: https://docs.rs/structopt/0.3.12/structopt/
/// You can see the code generated by `structopt` using `cargo expand`, see https://github.com/dtolnay/cargo-expand
///
/// Fill in the missing fields!
///
/// When you are ready, uncomment the appropriate lines from src/main.rs and
/// run `cargo run --bin jira-wip` in your terminal!
pub mod cli {
    use super::store_recap::{TicketStore, Status, TicketDraft, TicketPatch, TicketTitle, TicketDescription};
    use super::id_generation::TicketId;
    use std::error::Error;
    use std::str::FromStr;
    use std::fmt::Formatter;

    /*
    #[derive(structopt::StructOpt, Clone)]
    /// A small command-line interface to interact with a toy Jira clone, IronJira.
    pub enum Command {
        /// Create a ticket on your board.
        Create {
            /// Description of the ticket.
            #[structopt(long)]
            description: String,
            /// Title of your ticket - it cannot be empty!
            #[structopt(long)]
            title: String,
            /// Status of the new ticket.
            #[structopt(long)]
            status: Status,
        },
        /// Edit the details of an existing ticket.
        Edit {
            __
        },
        /// Delete a ticket from the store passing the ticket id.
        Delete {
            __
        },
        /// List all existing tickets.
        List,
    }
    */

    #[derive(structopt::StructOpt, Clone)]
    /// A small command-line interface to interact with a toy Jira clone, IronJira.
    pub enum Command {
        /// Create a ticket on your board.
        Create {
            /// Description of the ticket.
            #[structopt(long)]
            description: String,
            /// Title of your ticket - it cannot be empty!
            #[structopt(long)]
            title: String,
            /// Status of the new ticket.
            #[structopt(long)]
            status: Status,
        },
        /// Edit the details of an existing ticket.
        Edit {
            /// Id of the ticket you want to edit.
            #[structopt(long)]
            ticket_id: TicketId,
            /// New title for the ticket (optional).
            #[structopt(long)]
            title: Option<String>,
            /// New description for the ticket (optional).
            #[structopt(long)]
            description: Option<String>,
            /// New status for the ticket (optional).
            #[structopt(long)]
            status: Option<Status>,
        },
        /// Delete a ticket from the store passing the ticket id.
        Delete {
            /// Id of the ticket you want to delete.
            #[structopt(long)]
            ticket_id: TicketId,
        },
        /// List all existing tickets.
        List,
    }

    /// `structopt` relies on `FromStr` to know how to parse our custom structs and enums
    /// from the string passed in as input by a user.
    ///
    /// Parsing is fallible: we need to declare what error type we are going to return if
    /// things go wrong and implement the `from_str` function.
    impl FromStr for Status {
        type Err = ParsingError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let s = s.to_lowercase();
            match s.as_str() {
                "todo" | "to-do" => Ok(Status::ToDo),
                "inprogress" | "in-progress" => Ok(Status::InProgress),
                "blocked" => Ok(Status::Blocked),
                "done" => Ok(Status::Done),
                _ => Err(ParsingError("The status you specified is not valid. Valid values: todo, inprogress, blocked and done.".into())),
            }
        }

        /*
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            __
        }
        */
    }

    /// Our error struct for parsing failures.
    #[derive(Debug)]
    pub struct ParsingError(String);

    impl Error for ParsingError { }

    impl std::fmt::Display for ParsingError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "{}", self.0)
        }
    }

    /// The core function: given a mutable reference to a `TicketStore` and a `Command`,
    /// carry out the action specified by the user.
    /// We use `Box<dyn Error>` to avoid having to specify the exact failure modes of our
    /// top-level handler.
    ///
    /// `dyn Error` is the syntax of a trait object, a more advanced topic that we will not be
    /// touching in this workshop.
    /// Check its section in the Rust book if you are curious: https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
    /*
    fn handle_command(ticket_store: &mut TicketStore, command: Command) -> Result<(), Box<dyn Error>> {
        match command {
            Command::Create { description, title, status } => {
                // The ? operator can be used in functions that returns `Result` to return early
                // if a fallible operation didn't succeed.
                // It saves a bunch of lines of code as well as some visual branching.
                //
                // It might take a bit to get used to, but it will eventually become second-nature
                // if you keep writing and reading Rust code.
                // See https://doc.rust-lang.org/1.29.0/book/2018-edition/ch09-02-recoverable-errors-with-result.html#the--operator-can-only-be-used-in-functions-that-return-result
                // for more details.
                let draft = TicketDraft {
                    title: TicketTitle::new(title)?,
                    description: TicketDescription::new(description)?,
                    status,
                };
                ticket_store.save(draft);
            }
            Command::Edit {
                ticket_id,
                title,
                description,
                status,
            } => {
                __
            }
            Command::Delete { ticket_id } => match ticket_store.delete(&ticket_id) {
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
                __
            }
        }
        Ok(())
    }
    */

    pub fn handle_command(ticket_store: &mut TicketStore, command: Command) -> Result<(), Box<dyn Error>> {
        match command {
            Command::Create { description, title, status } => {
                // The ? operator can be used in functions that returns `Result` to return early
                // if a fallible operation didn't succeed.
                // It saves a bunch of lines of code as well as some visual branching.
                //
                // It might take a bit to get used to, but it will eventually become second-nature
                // if you keep writing and reading Rust code.
                // See https://doc.rust-lang.org/1.29.0/book/2018-edition/ch09-02-recoverable-errors-with-result.html#the--operator-can-only-be-used-in-functions-that-return-result
                // for more details.
                let draft = TicketDraft {
                    title: TicketTitle::new(title)?,
                    description: TicketDescription::new(description)?,
                    status,
                };
                ticket_store.save(draft);
            }
            Command::Edit {
                ticket_id,
                title,
                description,
                status,
            } => {
                let title = title.map(TicketTitle::new).transpose()?;
                let description = description.map(TicketDescription::new).transpose()?;
                let ticket_patch = TicketPatch {
                    title,
                    description,
                    status,
                };
                match ticket_store.update(&ticket_id, ticket_patch) {
                    Some(_) => println!("Ticket {:?} was updated.", ticket_id),
                    None => println!(
                        "There was no ticket associated to the ticket id {:?}",
                        ticket_id
                    ),
                }
            }
            Command::Delete { ticket_id } => match ticket_store.delete(&ticket_id) {
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
                let ticket_list = ticket_store
                    .list()
                    .into_iter()
                    .map(|t| format!("{:?}", t))
                    .collect::<Vec<String>>()
                    .join("\n\n");
                println!("{}", ticket_list);
            }
        }
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use fake::{Faker, Fake};

        #[test]
        fn invalid_status_fails_to_be_parsed()
        {
            let invalid_status = "Not a good status";
            assert!(Status::from_str(invalid_status).is_err());
        }
    }
}
