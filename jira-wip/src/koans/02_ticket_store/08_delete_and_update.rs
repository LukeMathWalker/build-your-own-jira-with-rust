mod delete_and_update {
    use std::collections::HashMap;
    use chrono::{DateTime, Utc};
    use super::recap::Status;
    use super::id_generation::TicketId;
    use std::error::Error;


    /// There are only two pieces missing: deleting a ticket and updating a ticket
    /// in our `TicketStore`.
    /// The update functionality will give us the possibility to change the `status` of
    /// a ticket, the holy grail of our JIRA clone.
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
            let timestamp = Utc::now();
            let ticket = Ticket {
                id,
                title: draft.title,
                description: draft.description,
                status: draft.status,
                created_at: timestamp.clone(),
                // A new field, to keep track of the last time a ticket has been touched.
                // It starts in sync with `created_at`, it gets updated when a ticket is updated.
                updated_at: timestamp,
            };
            self.data.insert(id, ticket);
            id
        }

        pub fn get(&self, id: &TicketId) -> Option<&Ticket> {
            self.data.get(id)
        }

        pub fn list(&self) -> Vec<&Ticket> {
            self.data.values().collect()
        }

        /// We take in an `id` and a `patch` struct: this allows us to constrain which
        /// of the fields in a `Ticket` can actually be updated.
        /// For example, we don't want users to be able to update the `id` or the `created_at` field.
        ///
        /// If we had chosen a different strategy, e.g. implementing a `get_mut` method
        /// to retrieve a mutable reference to a ticket and give the caller the possibility to edit
        /// it as they wanted, we wouldn't have been able to keep the same guarantees
        /// of immutability.
        ///
        /// If the `id` passed in matches a ticket in the store, we return the edited ticket.
        /// If it doesn't, we return `None`.
        pub fn update(&mut self, id: &TicketId, patch: TicketPatch) -> Option<&Ticket> {
            todo!()
        }

        /// If the `id` passed in matches a ticket in the store, we return the deleted ticket,
        /// with some additional metadata.
        /// If it doesn't, we return `None`.
        pub fn delete(&mut self, id: &TicketId) -> Option<DeletedTicket> {
            todo!()
        }

        fn generate_id(&mut self) -> TicketId {
            self.current_id += 1;
            self.current_id
        }
    }

    /// We don't want to relax our constraints on what is an acceptable title
    /// or an acceptable description for a ticket.
    /// This means that we need to validate the `title` and the `description` in
    /// our `TicketPatch` using the same rules we use for our `TicketDraft`.
    ///
    /// To keep it DRY, we introduce two new types whose constructors guarantee
    /// the invariant we care about.
    #[derive(Debug, Clone, PartialEq)]
    pub struct TicketTitle(String);

    impl TicketTitle {
        pub fn new(title: String) -> Result<Self, ValidationError> {
            if title.is_empty() {
                return Err(ValidationError("Title cannot be empty!".to_string()));
            }
            if title.len() > 50 {
                return Err(ValidationError("A title cannot be longer than 50 characters!".to_string()));
            }
            Ok(Self(title))
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct TicketDescription(String);

    impl TicketDescription {
        pub fn new(description: String) -> Result<Self, ValidationError> {
            if description.len() > 3000 {
                Err(ValidationError("A description cannot be longer than 3000 characters!".to_string()))
            } else {
                Ok(Self(description))
            }
        }
    }

    /// `TicketPatch` constrains the fields that we consider editable.
    ///
    /// If a field is set the `Some`, its value will be updated to the specified value.
    /// If a field is set to `None`, the field remains unchanged.
    #[derive(Debug, Clone, PartialEq)]
    pub struct TicketPatch {
        pub title: Option<TicketTitle>,
        pub description: Option<TicketDescription>,
        pub status: Option<Status>,
    }

    /// With validation baked in our types, we don't have to worry anymore about the visibility
    /// of those fields.
    /// Our `TicketPatch` and our `TicketDraft` don't have an identity, an id, like a `Ticket` saved in the store.
    /// They are value objects, not entities, to borrow some terminology from Domain Driven Design.
    ///
    /// As long as we know that our invariants are holding, we can let the user modify them
    /// as much as they please.
    /// We can thus get rid of the constructor and all the accessor methods. Pretty sweet, uh?
    #[derive(Debug, Clone, PartialEq)]
    pub struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
        pub status: Status,
    }

    /// A light wrapper around a deleted ticket, to store some metadata (the deletion timestamp).
    /// If we had a user system in place, we would also store the identity of the user
    /// who performed the deletion.
    #[derive(Debug, Clone, PartialEq)]
    pub struct DeletedTicket {
        ticket: Ticket,
        deleted_at: DateTime<Utc>,
    }

    impl DeletedTicket {
        pub fn ticket(&self) -> &Ticket { &self.ticket }
        pub fn deleted_at(&self) -> &DateTime<Utc> { &self.deleted_at }
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
        title: TicketTitle,
        description: TicketDescription,
        status: Status,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    }

    impl Ticket {
        pub fn title(&self) -> &TicketTitle { &self.title }
        pub fn description(&self) -> &TicketDescription { &self.description }
        pub fn status(&self) -> &Status { &self.status }
        pub fn created_at(&self) -> &DateTime<Utc> { &self.created_at }
        pub fn id(&self) -> &TicketId { &self.id }
        pub fn updated_at(&self) -> &DateTime<Utc> { &self.updated_at }
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        use fake::{Faker, Fake};
        use std::time::Duration;

        #[test]
        fn updating_nothing_leaves_the_updatable_fields_unchanged()
        {
            let mut store = TicketStore::new();
            let draft = generate_ticket_draft(Status::ToDo);
            let ticket_id = store.save(draft.clone());

            let patch = TicketPatch {
                title: None,
                description: None,
                status: None,
            };
            let updated_ticket = store.update(&ticket_id, patch).unwrap();

            assert_eq!(draft.title, updated_ticket.title);
            assert_eq!(draft.description, updated_ticket.description);
            assert_eq!(draft.status, updated_ticket.status);
        }

        #[test]
        fn trying_to_update_a_missing_ticket_returns_none()
        {
            let mut store = TicketStore::new();
            let ticket_id = Faker.fake();
            let patch = generate_ticket_patch(Status::Done);

            assert_eq!(store.update(&ticket_id, patch), None);
        }

        #[test]
        fn update_works()
        {
            let mut store = TicketStore::new();
            let draft = generate_ticket_draft(Status::ToDo);
            let patch = generate_ticket_patch(Status::Done);
            let ticket_id = store.save(draft.clone());

            // Let's wait a bit, otherwise `created_at` and `updated_at`
            // might turn out identical (ᴗ˳ᴗ)
            std::thread::sleep(Duration::from_millis(100));
            let updated_ticket = store.update(&ticket_id, patch.clone()).unwrap();

            assert_eq!(patch.title.unwrap(), updated_ticket.title);
            assert_eq!(patch.description.unwrap(), updated_ticket.description);
            assert_eq!(patch.status.unwrap(), updated_ticket.status);
            assert_ne!(updated_ticket.created_at(), updated_ticket.updated_at());
        }

        #[test]
        fn delete_works()
        {
            let mut store = TicketStore::new();
            let draft = generate_ticket_draft(Status::ToDo);
            let ticket_id = store.save(draft.clone());
            let ticket = store.get(&ticket_id).unwrap().to_owned();

            let deleted_ticket = store.delete(&ticket_id).unwrap();

            assert_eq!(deleted_ticket.ticket(), &ticket);
            assert_eq!(store.get(&ticket_id), None);
        }

        #[test]
        fn deleting_a_missing_ticket_returns_none()
        {
            let mut store = TicketStore::new();
            let ticket_id = Faker.fake();

            assert_eq!(store.delete(&ticket_id), None);
        }

        #[test]
        fn list_returns_all_tickets()
        {
            let n_tickets = 100;
            let mut store = TicketStore::new();

            for _ in 0..n_tickets {
                let draft = generate_ticket_draft(Status::ToDo);
                store.save(draft);
            }

            assert_eq!(n_tickets, store.list().len());
        }

        #[test]
        fn on_a_single_ticket_list_and_get_agree()
        {
            let mut store = TicketStore::new();

            let draft = generate_ticket_draft(Status::ToDo);
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
            assert!(TicketTitle::new("".into()).is_err())
        }

        #[test]
        fn title_cannot_be_longer_than_fifty_chars() {
            // Let's generate a title longer than 51 chars.
            let title = (51..10_000).fake();

            assert!(TicketTitle::new(title).is_err())
        }

        #[test]
        fn description_cannot_be_longer_than_3000_chars() {
            let description = (3001..10_000).fake();

            assert!(TicketDescription::new(description).is_err())
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
            assert_eq!(retrieved_ticket.created_at(), retrieved_ticket.updated_at());
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

        #[test]
        fn ids_are_not_reused()
        {
            let n_tickets = 100;
            let mut store = TicketStore::new();

            for expected_id in 1..n_tickets {
                let draft = generate_ticket_draft(Status::ToDo);
                let ticket_id = store.save(draft);
                assert_eq!(expected_id, ticket_id);
                assert!(store.delete(&ticket_id).is_some());
            }
        }

        fn generate_ticket_draft(status: Status) -> TicketDraft {
            let description = TicketDescription::new((0..3000).fake()).unwrap();
            let title = TicketTitle::new((1..50).fake()).unwrap();

            TicketDraft {
                title,
                description,
                status
            }
        }

        fn generate_ticket_patch(status: Status) -> TicketPatch {
            let patch = generate_ticket_draft(status);

            TicketPatch {
                title: Some(patch.title),
                description: Some(patch.description),
                status: Some(patch.status),
            }
        }
    }
}
