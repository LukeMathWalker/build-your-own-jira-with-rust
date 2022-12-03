/// Using modules and visibility modifiers we have now fully encapsulated the fields of our Ticket.
/// There is no way to create a Ticket instance skipping our validation.
/// At the same time though, we have made it impossible to access the fields of our struct,
/// because they are private!
///
/// Let's fix that introducing a bunch of accessor methods providing **read-only** access
/// to the fields in a ticket.

/// Let's import the Status enum we defined in the previous exercise, we won't have to modify it.
use super::visibility::ticket::Status;

/// Re-defining Ticket here because methods who need to access private fields
/// have to be defined in the same module of the struct itself, as we saw in the previous
/// exercise.
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

/// Methods on a struct are defined in `impl` blocks.
impl Ticket {
    /// The syntax looks very similar to the syntax to define functions.
    /// There is only one peculiarity: if you want to access the struct in a method,
    /// you need to take `self` as your first parameter in the method signature.
    ///
    /// You have three options, depending on what you are trying to accomplish:
    /// - self
    /// - &self
    /// - &mut self
    ///
    /// We are now touching for the first time the topic of ownership, enforced by
    /// the compiler via the (in)famous borrow-checker.
    ///
    /// In Rust, each value has an owner, statically determined at compile-time.
    /// There is only one owner for each value at any given time.
    /// Tracking ownership at compile-time is what makes it possible for Rust not to have
    /// garbage collection without requiring the developer to manage memory explicitly
    /// (most of the times).
    ///
    /// What can an owner do with a value `a`?
    /// It can mutate it.
    /// It can move ownership to another function or variable.
    /// It can lend many immutable references (`&a`) to that value to other functions or variables.
    /// It can lend a **single** mutable reference (`&mut a`) to that value to another
    /// function or variable.
    ///
    /// What can you do with a shared immutable reference (`&a`) to a value?
    /// You can read the value and create more immutable references.
    ///
    /// What can you do with a single mutable reference (`&mut a`) to a value?
    /// You can mutate the underlying value.
    ///
    /// Ownership is embedded in the type system: each function has to declare in its signature
    /// what kind of ownership level it requires for all its arguments.
    /// If the caller cannot fulfill those requirements, they cannot call the function.
    ///
    /// In our case, we only need to read a field of our Ticket struct: it will be enough to ask
    /// for an immutable reference to our struct.
    ///
    /// If this sounds a bit complicated/vague, hold on: it will get clearer as you
    /// move through the exercises and work your way through a bunch of compiler errors:
    /// the compiler is the best pair programming buddy to get familiar with ownership
    /// and its rules.
    /// To read more on ownership check:
    /// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html
    pub fn title(&self) -> &String {
        /// We are returning an immutable reference (&) to our title field.
        /// This will allow us to access this field without being able to mutate it:
        /// encapsulation is guaranteed and we can rest assured that our invariants
        /// cannot be violated.
        &self.title
    }

    /// Replace __ with the proper types to get accessor methods for the other two fields.
    /// If you are asking yourself why we are returning &str instead of &String, check out:
    /// https://blog.thoughtram.io/string-vs-str-in-rust/
    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }
}

pub fn create_ticket(title: String, description: String, status: Status) -> Ticket {
    if title.is_empty() {
        panic!("Title cannot be empty!");
    }
    if title.len() > 50 {
        panic!("A title cannot be longer than 50 characters!");
    }
    if description.len() > 3000 {
        panic!("A description cannot be longer than 3000 characters!");
    }

    Ticket {
        title,
        description,
        status,
    }
}

#[cfg(test)]
mod tests {
    use super::super::visibility::ticket::Status;
    use super::{create_ticket, Ticket};

    fn verify_without_tampering() {
        let ticket: Ticket = create_ticket("A title".into(), "A description".into(), Status::ToDo);

        /// Instead of accessing the field `ticket.description` we are calling the accessor
        /// method, `ticket.description()`, which returns us a reference to the field value
        /// and allows us to verify its value without having the chance to modify it.
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.title(), "A title");
    }
}
