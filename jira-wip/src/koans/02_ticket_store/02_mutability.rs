mod mutability {
    /// When creating our TicketStore we encountered a new keyword: mut.
    /// Mutability, the ability to change something, works a bit differently in Rust than in other languages.
    /// The default status for variables is that they are not mutable, but we can change this with the mut keyword.
    ///
    /// The keyword signals that you must pay special attention to the variable as it's likely to change later on.
    /// But changing the value a variable references does not necessarily mean changing the variables type.
    /// Let's explore what mutability means in Rust...


    #[cfg(test)]
    mod tests {

        /// Here we show a basic example of the 'mut' keyword.
        /// We declare the mutable variable x and initialise it to a value of 4
        /// After, we change the value to 5 and test that the value is indeed 5.
        #[test]
        fn to_change_a_value()
        {
            // try removing 'mut' keyword and see what happens
            // will the compiler even allow you to set the new value?
            let mut x = 4;
            x = 5;

            assert_eq!(5, x);
        }

        /// We have seen that we can change the value of a variable.
        /// But just because we can mutate the variable does not mean we can change it to whatever we like.
        /// Rust has a robust type system that is rigorously enforced by the compiler.
        #[test]
        fn that_which_cannot_be_changed()
        {
            // Try uncommenting the below line of code and see how the compiler assists you.
            // Think about how can we change x to make the compiler happy, and the test to pass.
            let mut x = 4;
            // x = "A new value, a new day.";

            /*
            todo
            assert_eq!("A new value, a new day", x);
            */
        }

        /// References can also be mutable.
        /// Here we assign a mutable reference to the variable y.
        ///
        /// Now y is an immutable binding to a mutable reference.
        /// That means that you can’t change y to some other mutable reference such as 'y = &mut z',
        /// but you can mutate the thing that’s bound to y. E.g '*y = 5'
        /// A subtle distinction that will take time to master.
        #[test]
        fn a_change_in_reference()
        {

            let mut x = 5;
            let y = &mut x;

            assert_eq!(5, *y);
        }

        // This is very basic of Rust's mutability,and there is a lot more to discover.
        // To lean more about mutability in Rust take a look at the official documentation: https://doc.rust-lang.org/1.0.0/book/mutability.html
    }
}