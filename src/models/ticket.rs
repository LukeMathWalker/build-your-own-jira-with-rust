#[derive(PartialEq, Debug, Clone)]
/// A ticket saved in the [TicketStore](TicketStore).
pub struct Ticket {
    pub id: u64,
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