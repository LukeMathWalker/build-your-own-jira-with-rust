/*
pub struct Ticket {
    title: String,
    __
}
*/

/// We will begin our journey of building our own JIRA clone defining
/// the cornerstone of JIRA's experience: the ticket.
/// For now we want to limit ourselves to the essentials: each ticket will have a title and a description.
/// No, not an ID yet. We will get to that in due time.
///
/// There are various ways to represent a set of related pieces of information in Rust.
/// We'll go for a `struct`: a struct is quite similar to what you would call a class or an object in
/// object-oriented programming languages.
/// It is a collection of fields, each one with its one name.
/// Given that Rust is a strongly-typed language, we also need to specify a type for each of those fields.
///
/// Our definition of Ticket is incomplete - can you replace __ with what is missing to make this snippet compile
/// and the tests below succeed?
///
/// You can find more about structs in the Rust Book: https://doc.rust-lang.org/book/ch05-01-defining-structs.html
pub struct Ticket {
    title: String,
    description: String
}

/// `cfg` stands for configuration flag.
/// The #[cfg(_)] attribute is used to mark a section of the code for conditional compilation
/// based on the value of the specified flag.
/// #[cfg(test)] is used to mark sections of our codebase that should only be compiled when running `cargo test`...
/// Yes, tests!
///
/// You can put tests in different places in a Rust project, depending on what you are trying to do: unit testing of
/// private functions and methods, testing an internal API, integration testing your crate from the outside, etc.
/// You can find more details on test organisation in the Rust book: https://doc.rust-lang.org/book/ch11-03-test-organization.html
///
/// Let it be said that tests are first-class citizens in the Rust ecosystem and you are provided with
/// a barebone test framework out of the box.
#[cfg(test)]
mod ticket {
    use super::*;

    /// The #[test] attribute is used to mark a function as test for the compiler.
    /// Tests take no arguments: when we run `cargo test`, this function will be invoked.
    /// If it runs without raising any issue, the test is considered green - it passed.
    /// If it panics (raises a fatal exception), then the test is considered red - it failed.
    ///
    /// `cargo test` reports on the number of failed tests at the end of each run, with some
    /// associated diagnostics to make it easier to understand what went wrong exactly.
    #[test]
    fn your_first_ticket() {
        let ticket_one : Ticket = Ticket {
            title: "A ticket title".into(),
            description: "A heart-breaking description".into()
        };

        assert_eq!(ticket_one.title, "A ticket title");
        assert_eq!(ticket_one.description, "A heart-breaking description");
    }
}