use crate::models::{DeletedTicket, Status, Ticket, TicketDraft, TicketId};
use std::collections::HashMap;

/// In-memory database where we store the saved [`Ticket`]s.
pub struct TicketStore {
    /// Current state of the internal sequence, used for id generation in generate_id.
    current_id: u64,
    /// The collection of stored tickets.
    data: HashMap<TicketId, Ticket>,
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
    pub fn create(&mut self, draft: TicketDraft) -> TicketId {
        let id = self.generate_id();
        let ticket = Ticket {
            id,
            description: draft.description,
            title: draft.title.get_title().to_string(),
            status: Status::ToDo,
        };
        self.data.insert(ticket.id, ticket);
        id
    }

    /// Remove a [Ticket] from the store.
    /// Returns None if the [Ticket](Ticket) is not there or [DeletedTicket](DeletedTicket) if there was one.
    pub fn delete(&mut self, ticket_id: TicketId) -> Option<DeletedTicket> {
        self.data.remove(&ticket_id).map(DeletedTicket)
    }

    /// Returns list off all inserted [Ticket](Ticket)
    /// Returns an empty list of tickets is there are no tickets in the store
    pub fn list(&self) -> Vec<&Ticket> {
        self.data.iter().map(|(_, ticket)| ticket).collect()
    }

    /// Generate a unique id by incrementing monotonically a private counter.
    fn generate_id(&mut self) -> TicketId {
        self.current_id += 1;
        self.current_id
    }

    /// Retrieve a [Ticket] given an identifier. Returns `None` if there is no ticket with such an identifier.
    pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
        self.data.get(id)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{Status, Ticket, TicketDraft, Title};
    use crate::store::TicketStore;
    use fake::Fake;
    use std::collections::HashSet;

    #[test]
    fn create_ticket_test() {
        let faker = fake::Faker;

        //arrange
        let draft = TicketDraft {
            title: Title::new(faker.fake()).expect("Title should exist"),
            description: faker.fake(),
        };

        let mut ticket_store = TicketStore::new();

        //act
        let ticket_id = ticket_store.create(draft.clone());

        //assert
        let ticket = ticket_store
            .get(&ticket_id)
            .expect("Failed to retrieve ticket.");
        assert_eq!(ticket.title, draft.title.get_title().to_string());
        assert_eq!(ticket.description, draft.description);
        assert_eq!(ticket.status, Status::ToDo);
    }

    #[test]
    fn delete_ticket_test() {
        let faker = fake::Faker;

        //arrange
        let draft = TicketDraft {
            title: Title::new(faker.fake()).expect("Title should exist"),
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
        assert_eq!(deleted_ticket.0, inserted_ticket);
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

    #[test]
    fn listing_tickets_of_an_empty_store_returns_an_empty_collection() {
        // Arrange
        let ticket_store = TicketStore::new();

        // Act
        let tickets = ticket_store.list();

        // Assert
        assert!(tickets.is_empty())
    }

    #[test]
    fn listing_tickets_should_return_them_all() {
        let faker = fake::Faker;

        // Arrange
        let mut ticket_store = TicketStore::new();
        let n_tickets = faker.fake::<u16>() as usize;
        let tickets: HashSet<_> = (0..n_tickets)
            .map(|_| generate_and_persist_ticket(&mut ticket_store))
            .collect();

        // Act
        let retrieved_tickets = ticket_store.list();

        // Assert
        assert_eq!(retrieved_tickets.len(), n_tickets);
        let retrieved_tickets: HashSet<_> = retrieved_tickets.into_iter().map(|t| t.to_owned()).collect();
        assert_eq!(tickets, retrieved_tickets);
    }

    fn generate_and_persist_ticket(store: &mut TicketStore) -> Ticket {
        let faker = fake::Faker;

        let draft = TicketDraft {
            title: Title::new(faker.fake()).expect("Title should exist"),
            description: faker.fake(),
        };
        let ticket_id = store.create(draft);
        store
            .get(&ticket_id)
            .expect("Failed to retrieve ticket")
            .to_owned()
    }
}