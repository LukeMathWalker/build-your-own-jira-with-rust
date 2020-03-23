mod traits {
    use crate::path_to_enlightenment::visibility::ticket::Status;

    /// You might have noticed that in the test for the previous koan we haven't checked if
    /// the status returned by `.status()` matched the status we passed to `create_ticket`.
    /// 
    /// That's because `assert_eq!(ticket.status(), Status::ToDo)` would have failed to compiled:
    /// 
    /// error[E0369]: binary operation `==` cannot be applied to type `&path_to_enlightenment::visibility::ticket::Status`
    ///    --> jira-wip/src/koans/01_ticket/05_ownership.rs:128:13
    ///    |
    /// 128 |             assert_eq!(ticket.status(), Status::ToDo);
    ///    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ///    |             |
    ///    |             &path_to_enlightenment::visibility::ticket::Status
    ///    |             path_to_enlightenment::visibility::ticket::Status
    ///    |
    ///    = note: an implementation of `std::cmp::PartialEq` might be missing for `&path_to_enlightenment::visibility::ticket::Status`
    ///
    /// `assert_eq` requires that its arguments implement the `PartialEq` trait.
    /// What is a trait?
    /// Traits in Rust are very similar to interfaces in other programming languages: 
    /// a trait describes a behaviour/capability.
    /// For example:
    /// 
    /// ```
    /// pub trait Pay {
    ///     fn pay(self, amount: u64, currency: String) -> u64
    /// }
    /// ```
    ///
    /// In practical terms, a trait defines the signature of a collection of methods.
    /// To implement a trait, a struct or an enum have to implement those methods
    /// in an `impl Trait` block:
    /// 
    /// ```
    /// impl Pay for TaxPayer {
    ///     fn pay(self, amount: u64, currency: String) -> u64 {
    ///         todo!()
    ///     }
    /// }
    /// ```
    /// 
    /// `PartialEq` is the trait that powers the == operator.
    /// Its definition looks something like this (simplified):
    /// ```
    /// pub trait PartialEq {
    ///     fn eq(&self, other: &Self) -> bool
    /// }
    /// ```
    /// It's slightly more complicated, with generic parameters, to allow comparing different types.
    /// But let's roll with this simplified version for now.
    /// 
    /// Let's implement it for Status!
    impl PartialEq for Status {
        fn eq(&self, other: &Status) -> bool {
            // If you need to refresh the `match` syntax, checkout
            // https://doc.rust-lang.org/book/ch06-02-match.html
            match (self, other) {
                __
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_equality() {
            // Your goal is to make this test compile.
            assert_eq!(Status::ToDo == Status::ToDo, true);
            assert_eq!(Status::Done == Status::ToDo, false);
            assert_eq!(Status::InProgress == Status::ToDo, false);
            assert_eq!(Status::InProgress == Status::InProgress, true);
        }
    }
}
