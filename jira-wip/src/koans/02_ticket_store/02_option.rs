
mod option {
    use std::collections::HashMap;
    use super::recap::Ticket;

    struct TicketStore {
        data: HashMap<u32, Ticket>,
    }

    impl TicketStore {
        pub fn new() -> TicketStore
        {
            TicketStore {
                data: HashMap::new()
            }
        }

        pub fn save(&mut self, ticket: Ticket, id: u32)
        {
            self.data.insert(id, ticket);
        }

        /// Trying to implement `get` in the previous koan might have caused you
        /// some issues due to a signature mismatch: `get` on a HashMap returns 
        /// an `Option<&Ticket>`, not a `&Ticket`.
        ///
        /// What is an Option?
        ///
        /// In a nutshell, Rust does not have `null`: if a function returns a `Ticket`
        /// there is no way for that `Ticket` not to be there.
        /// If there is indeed the possibility of the function not being able to return a `Ticket`,
        /// we need to express it in its return type.
        /// That's where `Option` comes in (`Option` as in `Option`al, or at least that how I think about it).
        /// `Option` is an enum:
        /// 
        /// ```
        /// enum Option<T> {
        ///     Some(T),
        ///     None
        /// }
        /// ```
        /// `T` is a generic type parameter here: as we saw for HashMap, Rust allows you to be generic over the types
        /// in your container. 
        /// The `None` variant means that the value is missing.
        /// The `Some` variant instead tells you that you have a value.
        /// 
        /// There is no way you can use the value in an `Option` without first checking the variant,
        /// hence it is impossible to "forget" to handle `None` when writing code.
        /// The compiler obliges you to handle both the happy and the unhappy case.
        ///
        /// For more details on `Option`, there is an exhaustive introduction in the Rust book: 
        /// https://doc.rust-lang.org/1.29.0/book/2018-edition/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values
        pub fn get(&self, id: &u32) -> Option<&Ticket> {
            self.data.get(id)
        }

        /*
        pub fn get(&self, id: &u32) -> Option<&Ticket> {
            todo!()
        }
        */
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use super::super::recap::{create_ticket, Status};

        /// Now let's put our TicketStore to use
        ///
        /// We are going to create a ticket as we have previously done
        /// then store it in our TicketStore, and finally validate that
        /// the ticket we have saved in our store is indeed the same ticket
        #[test]
        fn a_ticket_with_a_home()
        {
            let ticket = create_ticket("A ticket title".into(), "An enlightened description".into(), Status::ToDo);
            let mut store = TicketStore::new();
            let ticket_id = 1;

            store.save(ticket.clone(), ticket_id);

            // Notice that, even when a ticket with the specified id exists in the store,
            // it's returned as the `Some` variant of an `Option<&Ticket>`.
            assert_eq!(store.get(&ticket_id), Some(&ticket));
        }

        /// We want our `get` method to return None now, instead of panicking 
        /// when looking for an id to which there is no ticket associated.
        #[test]
        fn a_missing_ticket()
        {
            let ticket_store = TicketStore::new();

            assert_eq!(ticket_store.get(&1), None);
        }
    }
}
