mod result {
    use std::collections::HashMap;
    use chrono::{DateTime, Utc};
    use super::recap::Status;
    use super::id_generation::TicketId;
    use std::error::Error;

    /// The structure of our code is coming along quite nicely: it looks and feels like idiomatic
    /// Rust and it models appropriately the domain we are tackling, JIRA.
    ///
    /// There is still something we can improve though: our validation logic when creating a new
    /// draft.
    /// Our previous function, `create_ticket_draft`, panicked when either the title or
    /// the description failed our validation checks.
    /// The caller has no idea that this can happen - the function signature looks quite innocent:
    /// ```
    /// pub fn create_ticket_draft(title: String, description: String, status: Status) -> TicketDraft {
    /// ```
    /// Panics are generally not "caught" by the caller: they are meant to be used for states
    /// that your program cannot recover from.
    ///
    /// For expected error scenarios, we can do a better job using `Result`:
    /// ```
    /// pub fn create_ticket_draft(title: String, description: String, status: Status) -> Result<TicketDraft, ValidationError> {
    /// ```
    /// `Result` is an enum defined in the standard library, just like `Option`.
    /// While `Option` encodes the possibility that some data might be missing, `Result`
    /// encodes the idea that an operation can fail.
    ///
    /// Its definition looks something like this:
    /// ```
    /// pub enum Result<T, E> {
    ///     Ok(T),
    ///     Err(E)
    /// }
    /// ```
    /// The `Ok` variant is used to return the outcome of the function if its execution was successful.
    /// The `Err` variant is used to return an error describing what went wrong.
    ///
    /// The error type, `E`, has to implement the `Error` trait from the standard library.
    /// Let's archive our old `create_ticket_draft` function and let's define a new
    /// `TicketDraft::new` method returning a `Result` to better set expectations with the caller.
    #[derive(Debug, Clone, PartialEq)]
    pub struct TicketDraft {
        title: String,
        description: String,
        status: Status,
    }

    impl TicketDraft {
        pub fn title(&self) -> &String { &self.title }
        pub fn description(&self) -> &String { &self.description }
        pub fn status(&self) -> &Status { &self.status }

        pub fn new(title: String, description: String, status: Status) -> Result<TicketDraft, ValidationError> {
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
                status,
            };
            Ok(draft)
        }

        /*
        pub fn new(title: String, description: String, status: Status) -> Result<TicketDraft, ValidationError> {
            if title.is_empty() {
                return Err(ValidationError("Title cannot be empty!".to_string()));
            }
            if title.len() > 50 {
                __
            }
            if description.len() > 3000 {
                __
            }

            let draft = TicketDraft {
                title,
                description,
                status,
            };
            Ok(draft)
        }
        */
    }

    /// Our error struct, to be returned when validation fails.
    /// It's a wrapper around a string, the validation error message.
    /// Structs without field names are called tuple structs, you can read more about them in the Rust book:
    /// https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
    #[derive(PartialEq, Debug, Clone)]
    pub struct ValidationError(String);

    impl ValidationError {
        fn new(msg: &str) -> Self {
            Self(msg.to_string())
        }
    }

    /// To use `ValidationError` as the `Err` variant in a `Result` we need to implement
    /// the `Error` trait.
    ///
    /// The `Error` trait requires that our struct implements the `Debug` and `Display` traits,
    /// because errors might be bubbled up all the way until they are shown to the end user.
    /// We can derive `Debug`, but `Display` has to be implemented explicitly:
    /// `Display` rules how your struct is printed out for user-facing input, hence it cannot be
    /// derived.
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
                status: draft.status,
                created_at: Utc::now(),
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
        fn title_cannot_be_empty() {
            let description = (0..3000).fake();

            let result = TicketDraft::new("".into(), description, Status::ToDo);
            assert!(result.is_err())
        }

        #[test]
        fn title_cannot_be_longer_than_fifty_chars() {
            let description = (0..3000).fake();
            // Let's generate a title longer than 51 chars.
            let title = (51..10_000).fake();

            let result = TicketDraft::new(title, description, Status::ToDo);
            assert!(result.is_err())
        }

        #[test]
        fn description_cannot_be_longer_than_3000_chars() {
            let description = (3000..10_000).fake();
            let title = (0..50).fake();

            let result = TicketDraft::new(title, description, Status::ToDo);
            assert!(result.is_err())
        }

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

            TicketDraft::new(title, description, status).expect("Failed to create ticket")
        }
    }
}
