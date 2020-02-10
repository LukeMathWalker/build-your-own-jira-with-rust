use crate::models::{Comment, Title};
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::{Deserialize, Serialize};

pub type TicketId = u64;

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
/// A ticket saved in the [TicketStore](TicketStore).
///
/// **Invariant**: you can only build a ticket instance by retrieving it
/// from the [TicketStore](TicketStore).
#[derive(Serialize, Deserialize)]
pub struct Ticket {
    /// The id of the ticket. Randomly generated from the [TicketStore](TicketStore), guaranteed to be unique.
    pub id: TicketId,
    pub title: Title,
    pub description: String,
    pub status: Status,
    pub comments: Vec<Comment>,
}

impl std::fmt::Display for Ticket {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(
            f,
            "Ticket:\n\tId:{:?}\n\tTitle:{}\n\tDescription:{}\n\tStatus:{:?}\n\tComments:{:?}",
            self.id, self.title, self.description, self.status, self.comments
        )
    }
}

/// The status of a [Ticket](Ticket).
#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Blocked,
    Done,
}

#[derive(PartialEq, Debug)]
/// A ticket that was deleted from the store.
///
/// Using the new-type pattern to distinguish it from [Ticket](Ticket).
pub struct DeletedTicket(pub Ticket);
