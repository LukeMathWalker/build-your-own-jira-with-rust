mod vec {
    use std::collections::HashMap;
    use chrono::{DateTime, Utc};
    use super::recap::Status;
    use super::id_generation::TicketId;
    use std::error::Error;


    /// Let's turn our attention again to our `TicketStore`.
    /// We can create a ticket, we can retrieve a ticket.
    ///
    /// Let's implement a `list` method to retrieve all tickets currently in the store.
    struct TicketStore {
        data: HashMap<TicketId, Ticket>,
        current_id: TicketId,
    }

    impl TicketStore {
        pub fn new() -> TicketStore
        {
            TicketStore {
                data: HashMap::new(),
                current_id: 0,
            }
        }

        pub fn save(&mut self, draft: TicketDraft) -> TicketId
        {
            let id = self.generate_id();
            let ticket = Ticket {
                id,
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
                created_at: Utc::now(),
            };
            self.data.insert(id, ticket);
            id
        }

        pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
            self.data.get(id)
        }

        /// List will return a `Vec`.
        /// Check the Rust book for a primer: https://doc.rust-lang.org/book/ch08-01-vectors.html
        /// The Rust documentation for HashMap will also be handy: https://doc.rust-lang.org/std/collections/struct.HashMap.html
        pub fn list(&self) -> Vec<&Ticket> {
            __
        }

        fn generate_id(&mut self) -> TicketId {
            self.current_id += 1;
            self.current_id
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TicketDraft {
        title: String,
        description: String,
    }

    impl TicketDraft {
        pub fn title(&self) -> &String { &self.title }
        pub fn description(&self) -> &String { &self.description }

        pub fn new(title: String, description: String) -> Result<TicketDraft, ValidationError> {
            if title.is_empty() {
                return Err(ValidationError("Title cannot be empty!".to_string()));
            }
            if title.len() > 50 {
                return Err(ValidationError("A title cannot be longer than 50 characters!".to_string()));
            }
            if description.len() > 3000 {
                return Err(ValidationError("A description cannot be longer than 3000 characters!".to_string()));
            }

            let draft = TicketDraft {
                title,
                description,
            };
            Ok(draft)
        }
    }

    #[derive(PartialEq, Debug, Clone)]
    pub struct ValidationError(String);

    impl ValidationError {
        fn new(msg: &str) -> Self {
            Self(msg.to_string())
        }
    }

    impl Error for ValidationError { }

    impl std::fmt::Display for ValidationError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Ticket {
        id: TicketId,
        title: String,
        description: String,
        status: Status,
        created_at: DateTime<Utc>,
    }

    impl Ticket {
        pub fn title(&self) -> &String { &self.title }
        pub fn description(&self) -> &String { &self.description }
        pub fn status(&self) -> &Status { &self.status }
        pub fn created_at(&self) -> &DateTime<Utc> { &self.created_at }
        pub fn id(&self) -> &TicketId { &self.id }
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        use fake::{Faker, Fake};

        #[test]
        fn list_returns_all_tickets()
        {
            let n_tickets = 100;
            let mut store = TicketStore::new();

            for _ in 0..n_tickets {
                let draft = generate_ticket_draft();
                store.save(draft);
            }

            assert_eq!(n_tickets, store.list().len());
        }

        #[test]
        fn on_a_single_ticket_list_and_get_agree()
        {
            let mut store = TicketStore::new();

            let draft = generate_ticket_draft();
            let id = store.save(draft);

            assert_eq!(vec![store.get(&id).unwrap()], store.list());
        }

        #[test]
        fn list_returns_an_empty_vec_on_an_empty_store()
        {
            let store = TicketStore::new();

            assert!(store.list().is_empty());
        }

        #[test]
        fn title_cannot_be_empty() {
            let description = (0..3000).fake();

            let result = TicketDraft::new("".into(), description);
            assert!(result.is_err())
        }

        #[test]
        fn title_cannot_be_longer_than_fifty_chars() {
            let description = (0..3000).fake();
            // Let's generate a title longer than 51 chars.
            let title = (51..10_000).fake();

            let result = TicketDraft::new(title, description);
            assert!(result.is_err())
        }

        #[test]
        fn description_cannot_be_longer_than_3000_chars() {
            let description = (3001..10_000).fake();
            let title = (0..50).fake();

            let result = TicketDraft::new(title, description);
            assert!(result.is_err())
        }

        #[test]
        fn a_ticket_with_a_home()
        {
            let draft = generate_ticket_draft();
            let mut store = TicketStore::new();

            let ticket_id = store.save(draft.clone());
            let retrieved_ticket = store.get(&ticket_id).unwrap();

            assert_eq!(&ticket_id, retrieved_ticket.id());
            assert_eq!(&draft.title, retrieved_ticket.title());
            assert_eq!(&draft.description, retrieved_ticket.description());
            assert_eq!(&Status::ToDo, retrieved_ticket.status());
        }

        #[test]
        fn a_missing_ticket()
        {
            let ticket_store = TicketStore::new();
            let ticket_id = Faker.fake();

            assert_eq!(ticket_store.get(&ticket_id), None);
        }

        #[test]
        fn id_generation_is_monotonic()
        {
            let n_tickets = 100;
            let mut store = TicketStore::new();

            for expected_id in 1..n_tickets {
                let draft = generate_ticket_draft();
                let ticket_id = store.save(draft);
                assert_eq!(expected_id, ticket_id);
            }
        }

        fn generate_ticket_draft() -> TicketDraft {
            let description = (0..3000).fake();
            let title = (1..50).fake();

            TicketDraft::new(title, description).expect("Failed to create ticket")
        }
    }
}
