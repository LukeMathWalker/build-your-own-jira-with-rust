mod type_as_constraints {
    use std::collections::HashMap;
    use chrono::{DateTime, Utc};
    use super::recap::Status;
    use super::id_generation::TicketId;

    /// We know that id and creation time will never be there before a ticket is saved,
    /// while they will always be populated after `save` has been called.
    ///
    /// The approach we followed in the previous koan has its limitations: every time we
    /// access `id` and `created_at` we need to keep track of the "life stage" of our ticket.
    /// Has it been saved yet? Is it safe to unwrap whose `Option`s?
    /// That is unnecessary cognitive load and lead to errors down the line,
    /// when writing new code or refactoring existing functionality.
    ///
    /// We can do better.
    /// We can use types to better model our domain and constrain the behaviour of our code.
    ///
    /// Before `TicketStore::save` is called, we are dealing with a `TicketDraft`.
    /// No `created_at`, no `id`.
    /// On the other side, `TicketStore::get` will return a `Ticket`, with a `created_at` and
    /// an `id`.
    ///
    /// There will be no way to create a `Ticket` without passing through the store:
    /// we will enforce `save` as the only way to produce a `Ticket` from a `TicketDraft`.
    ///
    /// Less room for errors, less ambiguity, you can understand the domain constraints
    /// by looking at the signatures of the functions in our code.
    ///
    /// On the topic of type-driven development, checkout:
    /// - https://fsharpforfunandprofit.com/series/designing-with-types.html
    /// - https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/
    /// - https://www.youtube.com/watch?v=PLFl95c-IiU
    ///

    #[derive(Debug, Clone, PartialEq)]
    pub struct TicketDraft {
        __
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Ticket {
        __
    }

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

            // We can use the "raw" constructor for `Ticket` here because the
            // store is defined in the same module of `Ticket`.
            // If you are importing `Ticket` from another module,
            // `TicketStore::get` will indeed be the only way to get your hands on
            // an instance of `Ticket`.
            // This enforces our desired invariant: saving a draft in the store
            // is the only way to "create" a `Ticket`.
            let ticket = Ticket {

            };
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

    impl TicketDraft {
        pub fn title(&self) -> &String { todo!() }
        pub fn description(&self) -> &String { todo!() }
        pub fn status(&self) -> &Status { todo!() }
    }

    impl Ticket {
        pub fn title(&self) -> &String { todo!() }
        pub fn description(&self) -> &String { todo!() }
        pub fn status(&self) -> &Status { todo!() }
        pub fn created_at(&self) -> &DateTime<Utc> { todo!() }
        pub fn id(&self) -> &TicketId { todo!() }
    }

    pub fn create_ticket_draft(title: String, description: String, status: Status) -> TicketDraft {
        if title.is_empty() {
            panic!("Title cannot be empty!");
        }
        if title.len() > 50 {
            panic!("A title cannot be longer than 50 characters!");
        }
        if description.len() > 3000 {
            panic!("A description cannot be longer than 3000 characters!");
        }

        TicketDraft {
            title,
            description,
            status,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use fake::{Faker, Fake};

        #[test]
        fn a_ticket_with_a_home()
        {
            let draft = generate_ticket_draft(Status::ToDo);
            let mut store = TicketStore::new();

            let ticket_id = store.save(draft.clone());
            let retrieved_ticket = store.get(&ticket_id).unwrap();

            assert_eq!(&ticket_id, retrieved_ticket.id());
            assert_eq!(&draft.title, retrieved_ticket.title());
            assert_eq!(&draft.description, retrieved_ticket.description());
            assert_eq!(&draft.status, retrieved_ticket.status());
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
                let draft = generate_ticket_draft(Status::ToDo);
                let ticket_id = store.save(draft);
                assert_eq!(expected_id, ticket_id);
            }
        }

        fn generate_ticket_draft(status: Status) -> TicketDraft {
            let description = (0..3000).fake();
            let title = (1..50).fake();

            create_ticket_draft(title, description, status)
        }
    }
}