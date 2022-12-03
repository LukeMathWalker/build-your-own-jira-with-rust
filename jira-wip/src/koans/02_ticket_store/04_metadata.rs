use super::id_generation::TicketId;
use super::recap::Status;
/// `chrono` is the go-to crate in the Rust ecosystem when working with time.
/// `DateTime` deals with timezone-aware datetimes - it takes the timezone as a type parameter.
/// `DateTime<Utc>` is the type for datetimes expressed in the coordinated universal time.
/// See:
/// - https://en.wikipedia.org/wiki/Coordinated_Universal_Time
/// - https://docs.rs/chrono/0.4.11/chrono/
use chrono::{DateTime, Utc};
use std::collections::HashMap;

struct TicketStore {
    data: HashMap<TicketId, Ticket>,
    current_id: TicketId,
}

/// When we retrieve a ticket we saved, we'd like to receive with it a bunch of metadata:
/// - the generated id;
/// - the datetime of its creation.
///
/// Make the necessary changes without touching the types of the inputs and the returned
/// objects in our methods!
/// You can make inputs mutable, if needed.
impl TicketStore {
    pub fn new() -> TicketStore {
        TicketStore {
            data: HashMap::new(),
            current_id: 0,
        }
    }

    pub fn save(&mut self, mut ticket: Ticket) -> TicketId {
        let id = self.generate_id();
        ticket.id = Some(id);
        ticket.created_at = Some(Utc::now());
        self.data.insert(id, ticket);
        id
    }

    pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
        self.data.get(id)
    }

    fn generate_id(&mut self) -> TicketId {
        self.current_id += 1;
        self.current_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
    id: Option<TicketId>,
    created_at: Option<DateTime<Utc>>,
}

impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    // The datetime when the ticket was saved in the store, if it was saved.
    pub fn created_at(&self) -> Option<DateTime<Utc>> {
        self.created_at
    }

    // The id associated with the ticket when it was saved in the store, if it was saved.
    pub fn id(&self) -> Option<&TicketId> {
        match &self.id {
            None => None,
            Some(id) => Some(id),
        }
    }
}

pub fn create_ticket(title: String, description: String, status: Status) -> Ticket {
    if title.is_empty() {
        panic!("Title cannot be empty!");
    }
    if title.len() > 50 {
        panic!("A title cannot be longer than 50 characters!");
    }
    if description.len() > 3000 {
        panic!("A description cannot be longer than 3000 characters!");
    }

    Ticket {
        title,
        description,
        status,
        id: None,
        created_at: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};

    #[test]
    fn ticket_creation() {
        let ticket = generate_ticket(Status::ToDo);

        assert!(ticket.id().is_none());
        assert!(ticket.created_at().is_none());
    }

    #[test]
    fn a_ticket_with_a_home() {
        let ticket = generate_ticket(Status::ToDo);
        let mut store = TicketStore::new();

        let ticket_id = store.save(ticket.clone());
        let retrieved_ticket = store.get(&ticket_id).unwrap();

        assert_eq!(Some(&ticket_id), retrieved_ticket.id());
        assert_eq!(&ticket.title, retrieved_ticket.title());
        assert_eq!(&ticket.description, retrieved_ticket.description());
        assert_eq!(&ticket.status, retrieved_ticket.status());
        assert!(retrieved_ticket.created_at().is_some());
    }

    #[test]
    fn a_missing_ticket() {
        let ticket_store = TicketStore::new();
        let ticket_id = Faker.fake();

        assert_eq!(ticket_store.get(&ticket_id), None);
    }

    #[test]
    fn id_generation_is_monotonic() {
        let n_tickets = 100;
        let mut store = TicketStore::new();

        for expected_id in 1..n_tickets {
            let ticket = generate_ticket(Status::ToDo);
            let ticket_id = store.save(ticket);
            assert_eq!(expected_id, ticket_id);
        }
    }

    fn generate_ticket(status: Status) -> Ticket {
        let description = (0..3000).fake();
        let title = (1..50).fake();

        create_ticket(title, description, status)
    }
}
