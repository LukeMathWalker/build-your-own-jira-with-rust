mod status {
    /// Ticket have two purposes in JIRA: capturing information about a task and tracking the completion of
    /// the task itself.
    ///
    /// Let's add a new field to our `Ticket` struct, `status`.
    /// For the time being, we'll work under the simplified assumption that the set of statuses for a ticket
    /// is fixed and can't be customised by the user.
    /// A ticket is either in the to-do column, in progress, blocked or done.
    /// What is the best way to represent this information in Rust?
    struct Ticket {
        title: String,
        description: String,
        status: Status,
    }

    /// Rust's enums are perfect for this usecase.
    /// Enum stands for enumeration: a type encoding the constraint that only a finite set of values is possible.
    /// Enums are great to encode semantic information in your program: making domain constraints explicit.
    ///
    /// Each possible value of an enum is called a variant. By convention they are Pascal-cased.
    /// Check out the Rust book for more details on enums: https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
    ///
    /// Let's create a variant for each of the allowed statuses of our tickets.
    pub enum Status {
        ToDo,
        __
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn a_blocked_ticket() {
            // Let's create a blocked ticket.
            let ticket = Ticket {
                title: "A ticket title".into(),
                description: "A heart-breaking description".into(),
                status: __
            };

            // Let's check that the status corresponds to what we expect.
            // We can use pattern matching to take a different course of action based on the enum
            // variant we are looking at.
            // The Rust compiler will make sure that the match statement is exhaustive: it has to handle
            // all variants in our enums. If not, the compiler will complain and reject our program.
            //
            // This is extremely useful when working evolving codebases: if tomorrow we decide that tickets
            // can also have `Backlog` as their status, the Rust compiler will highlight all code locations
            // where we need to account for the new variant. No way to forget!
            //
            // Checkout the Rust Book for more details: https://doc.rust-lang.org/book/ch06-02-match.html
            match ticket.status {
                // Variant => Expression
                Status::Blocked => println!("Great, as expected!"),
                // If we want to take the same action for multiple variants, we can use a | to list them.
                // Variant | Variant | ... | Variant => Expression
                //
                // We are panicking in this case, thus making the test fail if this branch of our match
                // statement gets executed.
                Status::ToDo | Status::InProgress | Status::Done => panic!("The ticket is not blocked!")
            }
        }
    }
}
