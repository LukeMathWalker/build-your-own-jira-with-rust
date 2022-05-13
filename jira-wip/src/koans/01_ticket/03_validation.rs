enum Status {
    ToDo,
    InProgress,
    Blocked,
    Done,
}

struct Ticket {
    title: String,
    description: String,
    status: Status,
}

/// So far we have allowed any string as a valid title and description.
/// That's not what would happen in JIRA: we wouldn't allow tickets with an empty title,
/// for example.
/// Both title and description would also have length limitations: the Divine Comedy probably
/// shouldn't be allowed as a ticket description.
///
/// We want to define a function that takes in a title, a description and a status and
/// performs validation: it panics if validation fails, it returns a `Ticket` if validation
/// succeeds.
///
/// We will learn a better way to handle recoverable errors such as this one further along,
/// but let's rely on panic for the time being.
fn create_ticket(title: String, description: String, status: Status) -> Ticket {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::Fake;

    /// The #[should_panic] attribute inverts the usual behaviour for tests: if execution of
    /// the test's function body causes a panic, the test is green; otherwise, it's red.
    ///
    /// This is quite handy to test unhappy path: in our case, what happens when invalid input
    /// is passed to `create_ticket`.
    #[test]
    #[should_panic]
    fn title_cannot_be_empty() {
        // We don't really care about the description in this test.
        // Hence we generate a random string, with length between 0 and 3000 characters
        // using `fake`, a handy crate to generate random test data.
        //
        // We are using Rust's range syntax, 0..3000 - the lower-bound is included, the
        // upper-bound is excluded.
        // You can include the upper-bound using 0..=3000.
        let description = (0..3000).fake();

        create_ticket("".into(), description, Status::ToDo);
    }

    #[test]
    #[should_panic]
    fn title_cannot_be_longer_than_fifty_chars() {
        let description = (0..3000).fake();
        // Let's generate a title longer than 51 chars.
        let title = (51..10_000).fake();

        create_ticket(title, description, Status::ToDo);
    }

    #[test]
    #[should_panic]
    fn description_cannot_be_longer_than_3000_chars() {
        let description = (3001..10_000).fake();
        let title = (1..50).fake();

        create_ticket(title, description, Status::ToDo);
    }

    #[test]
    fn valid_tickets_can_be_created() {
        let description = (0..3000).fake();
        let title = (1..50).fake();
        let status = Status::Done;

        create_ticket(title, description, status);
    }
}
