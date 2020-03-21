mod id_generation {
    use std::collections::HashMap;
    use super::recap::Ticket;

    /// Let's define a type-alias for our TicketId.
    ///
    /// The underlying type remains u32.
    /// This remains valid code:
    /// ```
    /// let number: u32 = 1;
    /// let ticket_id: TicketId = number;
    /// ```
    /// It's a lightweight technique to add a semantic layer to the underlying data type.
    /// If we want to be sure we weren't mixing up ticket ids and `u32` variables with
    /// a different semantic meaning, we would have to create a new type, e.g. `struct TicketId(u32)`.
    /// For now this doesn't feel necessary - we don't have many u32 flying around.
    type TicketId = u32;

    // Feel free to add more fields to `TicketStore` to solve this koan!
    struct TicketStore {
        data: HashMap<TicketId, Ticket>,
        current_id: TicketId,
    }

    /*
    // Feel free to add more fields to `TicketStore` to solve this koan!
    struct TicketStore {
        data: HashMap<TicketId, Ticket>,
    }
    */

    impl TicketStore {
        pub fn new() -> TicketStore
        {
            TicketStore {
                data: HashMap::new(),
                current_id: 0,
            }
        }

        /// So far we have taken the `id` as one the parameters of our `save` method.
        ///
        /// What happens when you call save passing two different tickets with the same id?
        /// We have enforced with a test our expectation: the second ticket overwrites the first.
        /// The other option would have been to error out.
        ///
        /// This isn't our JIRA works: you don't get to choose the id of your ticket,
        /// it's generated for you and its uniqueness is guaranteed.
        /// There is also another peculiarity: ids are integers and they are monotonically
        /// increasing (the first ticket on a board will be `BOARDNAME-1`, the second
        /// `BOARDNAME-2` and so on).
        ///
        /// We want the same behaviour in our clone, IronJira.
        /// `TicketStore` will take of generating an id for our ticket and the id
        /// will be returned by `save` after insertion.
        pub fn save(&mut self, ticket: Ticket) -> TicketId
        {
            let id = self.generate_id();
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

        /*
        fn generate_id(__) -> TicketId {
            __
        }
        */
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use super::super::recap::{create_ticket, Status};
        use fake::{Faker, Fake};

        #[test]
        fn a_ticket_with_a_home()
        {
            let ticket = generate_ticket(Status::ToDo);
            let mut store = TicketStore::new();

            let ticket_id = store.save(ticket.clone());

            assert_eq!(store.get(&ticket_id), Some(&ticket));
            assert_eq!(ticket_id, 1);
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
}