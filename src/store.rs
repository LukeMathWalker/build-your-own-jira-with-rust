use std::collections::HashMap;
use crate::models::{Ticket, TicketDraft, Status};

/// In-memory database where we store the saved [`Ticket`]s.
pub struct TicketStore {
    /// Current state of the internal sequence, used for id generation in generate_id.
    current_id: u64,
    /// The collection of stored tickets.
    data: HashMap<u64, Ticket>,
}

impl TicketStore {
    /// Create a new empty [`TicketStore`] instance.
    pub fn new() -> Self {
        Self {
            current_id: 0,
            data: HashMap::new(),
        }
    }

    /// Given a ticket draft, it generates a unique identifier, it persists
    /// the new ticket in the store (assigning it a [ToDo status](Status::ToDo)) and returns
    /// the ticket identifier.
    pub fn create(&mut self, draft: TicketDraft) -> u64 {
        let id = self.generate_id();
        let ticket = Ticket {
            id,
            description: draft.description,
            title: draft.title,
            status: Status::ToDo,
        };
        self.data.insert(ticket.id, ticket);
        id
    }

    /// Generate a unique id by incrementing monotonically a private counter.
    fn generate_id(&mut self) -> u64 {
        self.current_id += 1;
        self.current_id
    }

    /// Retrieve a ticket given an identifier. Returns `None` if there is no ticket with such an identifier.
    pub fn get(&self, id: &u64) -> Option<&Ticket> {
        self.data.get(id)
    }
}