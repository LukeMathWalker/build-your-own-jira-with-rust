#[derive(PartialEq, Debug, Clone)]
/// The content of the ticket, not yet saved in the [TicketStore](TicketStore::create).
pub struct TicketDraft {
    pub title: String,
    pub description: String,
}
