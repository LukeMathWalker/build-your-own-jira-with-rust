use crate::models::Title;

#[derive(PartialEq, Debug, Clone)]
/// The content of the ticket, not yet saved in the [TicketStore](TicketStore::create).
pub struct TicketDraft {
    // The [Title](Title) of a ticket
    pub title: Title,
    pub description: String,
}
