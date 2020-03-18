mod store {
    /// So far on this journey of building our own JIRA clone we have managed to complete a quite a lot.
    /// From building a Ticket to updating the visibility of our modules.
    ///
    /// Now we shift focus. Our tickets are doing well, but they need a home.
    /// A place where we can store our tickets, search for them, and retrieve them.
    ///
    /// We can use many different data structures to store and manage our tickets,
    /// and it's wise to compare the pros and cons of data structures and select the best
    /// ones that fit the use case.
    ///
    /// For us we will have a ticket and some way to identify it, say with a unique identifier.
    /// So it makes sense for us to use a HashMap to store our tickets.
    /// You can read more about the HashMap in rust here: https://doc.rust-lang.org/std/collections/struct.HashMap.html
    /// They work in a similar manner to HashMaps in other languages.
    use std::collections::HashMap;

    /*
    pub struct TicketStore {
        /// The collection of stored tickets.
        ToDo
        data: ,
    }
    */

    /// First we will create a TicketStore for tickets.
    /// We can store an instance of the HashMap in a property of the TicketStore struct
    /// The HashMap takes two generic parameters, one for the key, and one for the value <K, V>
    ///
    /// Let's set the value to the type of our Ticket, and feel free to choose whatever type you
    /// would like for the key for now as we will update the key later.
    struct TicketStore {
        /// The collection of stored tickets.
        data: HashMap<u32, Ticket>,
    }

    /// Now we can call a function to create a TicketStore
    /// with a HashMap under the data property
    fn create_ticket_store() -> TicketStore
    {
        TicketStore{
            // Note that the compiler can infer the
            // types for our HashMaps' key value pairs.
            data: HashMap::new()
        }
    }

    struct Ticket {
        title: String,
        description: String
    }


    #[cfg(test)]
    mod tests {

        use super::*;

        /// Now let's put our TicketStore to use
        ///
        /// We are going to create a ticket as we have previously done
        /// then store it in our TicketStore, and finally validate that
        /// the ticket we have saved in our store is indeed the same ticket
        #[test]
        fn a_ticket_with_a_home()
        {
            let ticket = Ticket {
                title: "A ticket title".into(),
                description: "An enlightened description".into()
            };

            // Pay special attention to the 'mut' keyword here.
            // We have not encountered this keyword before, but don't worry
            // we will get to it in the next section.
            //
            // For now just note that if we wish to store anything in our HashMap
            // we require the mut keyword when creating our TicketStore
            let mut store = create_ticket_store();
            let ticket_id = 1;

            store.data.insert(ticket_id, ticket);

            assert_eq!(store.data.get(&ticket_id).expect("Could not find the ticket").title, "A ticket title");

        }

        /// Like HashMaps in other languages.
        /// If you try and force retrieve a value for a key that does not exist
        /// the HashMap will panic!
        /// So make sure the value is there.
        ///
        /// Don't worry, Rust has a way to handle cases
        /// where the HashMap tells us that a value does not exist.
        /// We will take a look at that scenario a little later.
        #[test]
        #[should_panic]
        fn a_missing_ticket()
        {
            let ticket = Ticket {
                title: "A ticket title".into(),
                description: "An enlightened description".into()
            };

            let mut store = create_ticket_store();

            store.data.insert(1, ticket);

            assert_eq!(store.data.get(&3).expect("Could not find the ticket").title, "A ticket title");


            /*
            assert_eq!(store.data.get(&1).expect("Could not find the ticket").description, "An enlightened description");
            */


        }

    }
}
