use crate::models::{Title, Comment};
pub type TicketId = u64;

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
/// A ticket saved in the [TicketStore](TicketStore).
///
/// **Invariant**: you can only build a ticket instance by retrieving it
/// from the [TicketStore](TicketStore).
pub struct Ticket {
    /// The id of the ticket. Randomly generated from the [TicketStore](TicketStore), guaranteed to be unique.
    pub id: TicketId,
    pub title: Title,
    pub description: String,
    pub status: Status,
    pub comments: Vec<Comment>,
}

#[derive(PartialEq, Debug, Clone, Hash, Eq)]
/// The status of a [Ticket](Ticket).
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
