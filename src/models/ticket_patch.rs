use crate::models::Title;

#[derive(PartialEq, Debug, Clone)]
/// The content of the ticket, to be updated in the [TicketStore](TicketStore::create).
pub struct TicketPatch {
    // The [Title](Title) of a ticket
    pub title: Option<Title>,
    pub description: Option<String>,
}
