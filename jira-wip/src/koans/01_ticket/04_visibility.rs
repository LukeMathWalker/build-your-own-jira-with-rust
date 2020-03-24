mod visibility {
    /// You might have noticed the `mod XX` at the beginning of our koans.
    /// `mod` stands for module: it's one of the tools Rust gives you to organise your code.
    /// In particular, modules have an impact on the visibility of your structs, enums and functions.
    ///
    /// We want to use this koan to explore the impact that modules have on the structure of your
    /// projects and how you can leverage them to enforce encapsulation.
    ///
    /// You can find out more about modules and visibility in the Rust book:
    /// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
    pub mod ticket {
        /// Structs, enums and functions defined in a module are visible to all other structs,
        /// enums and functions in the same module - e.g. we can use `Ticket` in the signature
        /// of `create_ticket` as our return type.
        ///
        /// That is no longer the case outside of the module where they are defined: all entities
        /// in Rust are private by default, unless they prefixed with `pub`.
        ///
        /// The same applies to fields in a struct.
        /// Functions defined within the same module of a struct have access to all the fields of
        /// the struct (e.g. `create_ticket` can create a `Ticket` by specifying its fields).
        /// Outside of the module, those fields are inaccessible because they are considered
        /// private by default, unless prefixed with pub.
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

        fn create_ticket(title: String, description: String, status: Status) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty!");
            }
            if title.len() > 50 {
                panic!("A title cannot be longer than 50 characters!");
            }
            if description.len() > 3000 {
                panic!("A description cannot be longer than 3000 characters!");
            }

            // Functions implicitly return the result of their last expression so we can omit
            // the `return` keyword here.
            Ticket {
                title,
                description,
                status,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        /// Add the necessary `pub` modifiers in the code above to avoid having the compiler
        /// complaining about this use statement.
        use super::ticket::{create_ticket, Status, Ticket};

        /// Be careful though! We don't want this function to compile after you have changed
        /// visibility to make the use statement compile!
        /// Once you have verified that it indeed doesn't compile, comment it out.
        fn should_not_be_possible() {
            let ticket: Ticket =
                create_ticket("A title".into(), "A description".into(), Status::ToDo);

            // You should be seeing this error when trying to run this koan:
            //
            // error[E0616]: field `description` of struct `path_to_enlightenment::visibility::ticket::Ticket` is private
            // --> jira-wip/src/koans/01_ticket/04_visibility.rs:99:25
            //    |
            // 99 |              assert_eq!(ticket.description, "A description");
            //    |                         ^^^^^^^^^^^^^^^^^^
            //
            // Once you have verified that the below does not compile,
            // comment the line out to move on to the next koan!
            assert_eq!(ticket.description, "A description");
        }

        fn encapsulation_cannot_be_violated() {
            // This should be impossible as well, with a similar error as the one encountered above.
            // (It will throw a compilation error only after you have commented the faulty line
            // in the previous test - next compilation stage!)
            //
            // This proves that `create_ticket` is now the only way to get a `Ticket` instance.
            // It's impossible to create a ticket with an illegal title or description!
            //
            // Once you have verified that the below does not compile,
            // comment the lines out to move on to the next koan!
            let ticket = Ticket {
                title: "A title".into(),
                description: "A description".into(),
                status: Status::ToDo,
            };
        }
    }
}
