use crate::models::{Status, Ticket, TicketDraft};
use std::collections::HashMap;

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

    /// Remove a [Ticket] from the store.
    /// Returns None if the [Ticket](Ticket) is not there.
    pub fn delete(&mut self, ticket_id: u64) -> Option<Ticket> {
        self.data.remove(&ticket_id)
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

#[cfg(test)]
mod tests {
    use crate::models::{Status, TicketDraft};
    use crate::store::TicketStore;
    use fake::Fake;

    #[test]
    fn create_ticket_test() {
        let faker = fake::Faker;

        //arrange
        let draft = TicketDraft {
            title: faker.fake(),
            description: faker.fake(),
        };

        let mut ticket_store = TicketStore::new();

        //act
        let ticket_id = ticket_store.create(draft.clone());

        //assert
        let ticket = ticket_store
            .get(&ticket_id)
            .expect("Failed to retrieve ticket.");
        assert_eq!(ticket.title, draft.title);
        assert_eq!(ticket.description, draft.description);
        assert_eq!(ticket.status, Status::ToDo);
    }

    #[test]
    fn delete_ticket_test() {
        let faker = fake::Faker;

        //arrange
        let draft = TicketDraft {
            title: faker.fake(),
            description: faker.fake(),
        };

        let mut ticket_store = TicketStore::new();
        let ticket_id = ticket_store.create(draft.clone());
        let inserted_ticket = ticket_store
            .get(&ticket_id)
            .expect("Failed to retrieve ticket")
            .to_owned();

        //act
        let deleted_ticket = ticket_store
            .delete(ticket_id)
            .expect("There was no ticket to delete.");

        //assert
        assert_eq!(deleted_ticket, inserted_ticket);
        let ticket = ticket_store.get(&ticket_id);
        assert_eq!(ticket, None);
    }

    #[test]
    fn deleting_a_ticket_that_does_not_exist_returns_none() {
        let faker = fake::Faker;

        //arrange
        let mut ticket_store = TicketStore::new();

        //act
        let deleted_ticket = ticket_store.delete(faker.fake());

        //assert
        assert_eq!(deleted_ticket, None);
    }
}
