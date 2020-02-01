pub type TicketId = u64;

#[derive(PartialEq, Debug, Clone)]
/// A ticket saved in the [TicketStore](TicketStore).
/// 
/// **Invariant**: you can only build a ticket instance by retrieving it
/// from the [TicketStore](TicketStore).
pub struct Ticket {
    /// The id of the ticket. Randomly generated from the [TicketStore](TicketStore), guaranteed to be unique. 
    pub id: TicketId,
    pub title: String,
    pub description: String,
    pub status: Status,
}

#[derive(PartialEq, Debug, Clone)]
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