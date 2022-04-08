/// It's time to shift focus: our tickets are doing well, but they need a home.
/// A place where we can store them, search for them, retrieve them.
///
/// We can use many different data structures to store and manage our tickets.
/// JIRA users rely heavily on ticket identifiers, e.g. RUST-2018 or COVID-19.
/// It's a unique label that unambiguously identifies a single ticket,
/// generally `<board name>-<ticket number>`.
/// We don't have the concept of a board yet, so we'll roll with a simple numerical id.
///
/// What is the simplest data structure that allows us to fetch a ticket given its id?
/// It makes sense for us to use a HashMap, also known as a dictionary in other languages.
/// You can read more about the HashMap in Rust here:
/// https://doc.rust-lang.org/std/collections/struct.HashMap.html
use std::collections::HashMap;
/// Let's import what we worked on in the previous set of exercises.
use super::recap::Ticket;

/// First we will create a TicketStore struct, with a `data` field of type HashMap.
///
/// HashMap is a *generic* struct: we need to specify two types, one for the key, and one for
/// the stored value - HashMap<K, V>.
///
/// Let's set the value type to our Ticket, and we will use an unsigned integer for our ids.
struct TicketStore {
    /// The collection of stored tickets.
    data: HashMap<u32, Ticket>,
}

impl TicketStore {
    /// Methods do not have to take self as a parameter.
    /// This is the equivalent of a class/static method in other programming languages.
    /// It can be invoked using `TicketStore::new()`.
    pub fn new() -> TicketStore {
        TicketStore {
            // Note that the compiler can infer the types for our HashMaps' key-value pairs.
            data: HashMap::new(),
        }
    }

    /// We take `&mut self` because we will have to mutate our HashMap to insert a new
    /// key-value pair.
    pub fn save(&mut self, ticket: Ticket, id: u32) {
        todo!()
    }

    pub fn get(&self, id: &u32) -> &Ticket {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::super::recap::{create_ticket, Status};
    use super::*;
    use fake::{Fake, Faker};

    /// Now let's put our TicketStore to use
    ///
    /// We are going to create a ticket, save it in our TicketStore and finally validate that
    /// the ticket we have saved in our store is indeed the same ticket we created.
    #[test]
    fn a_ticket_with_a_home() {
        let ticket = generate_ticket(Status::ToDo);

        // Pay special attention to the 'mut' keyword here: variables are immutable
        // by default in Rust.
        // The `mut` keyword is used to signal that you must pay special attention to the
        // variable as it's likely to change later on in the function body.
        let mut store = TicketStore::new();
        let ticket_id = Faker.fake();

        // Here we need to create a clone of our `ticket` because `save` takes the `ticket`
        // argument as value, thus taking ownership of its value out of the caller function
        // into the method.
        // But we need `ticket`'s value after this method call, to verify it matches what
        // we retrieve.
        // Hence the need to clone it, creating a copy of the value and passing that copy to
        // the `save` method.
        //
        // (You might have to go back to the `recap` koan to derive a couple more traits
        // for Ticket and Status...)
        store.save(ticket.clone(), ticket_id);

        assert_eq!(store.get(&ticket_id), &ticket);
    }

    /// We want our `get` method to panic when looking for an id to which there is no ticket
    /// associated (for now).
    ///
    /// Rust has a way to handle this failure mode more gracefully, we will take a look
    /// at it later.
    #[test]
    #[should_panic]
    fn a_missing_ticket() {
        let ticket_store = TicketStore::new();
        let ticket_id = Faker.fake();

        ticket_store.get(&ticket_id);
    }

    /// This is not our desired behaviour for the final version of the ticket store
    /// but it will do for now.
    #[test]
    fn inserting_a_ticket_with_an_existing_id_overwrites_previous_ticket() {
        let first_ticket = generate_ticket(Status::ToDo);
        let second_ticket = generate_ticket(Status::ToDo);
        let ticket_id = Faker.fake();
        let mut store = TicketStore::new();

        store.save(first_ticket.clone(), ticket_id);
        assert_eq!(store.get(&ticket_id), &first_ticket);

        store.save(second_ticket.clone(), ticket_id);
        assert_eq!(store.get(&ticket_id), &second_ticket);
    }

    fn generate_ticket(status: Status) -> Ticket {
        let description = (0..3000).fake();
        let title = (1..50).fake();

        create_ticket(title, description, status)
    }
}
