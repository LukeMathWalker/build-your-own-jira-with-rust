/*pub struct Ticket {
    __
}*/

pub struct Ticket {
    pub title: String,
    pub description: String
}

#[cfg(test)]
mod ticket {
    use super::*;

    #[test]
    /// Let's create a ticket
    /// Define the ticket struct above
    /// link to structs in docs?
    fn your_first_ticket() {
        let title = "test_title".to_string();
        let description = "test_description".to_string();

        let ticket_one : Ticket = Ticket {
            title: title.clone(),
            description: description.clone()
        };

        assert_eq!(ticket_one.title, title);
        assert_eq!(ticket_one.description, description);
    }
}