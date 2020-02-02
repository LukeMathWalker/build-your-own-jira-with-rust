use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
/// The content of the ticket, not yet saved in the [TicketStore](TicketStore::create).
pub struct TicketDraft {
    title: String,
    description: String
}

#[derive(Debug)]
pub struct TicketDraftError {
    details: String
}

impl TicketDraftError {
    fn new(msg: &str) -> TicketDraftError {
        TicketDraftError{details: msg.to_string()}
    }
}

impl fmt::Display for TicketDraftError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for TicketDraftError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl TicketDraft {
    pub fn new(title: String, description: String) -> Result<TicketDraft, TicketDraftError> {
        
        if title.is_empty() {
            Err(TicketDraftError::new(""))
        } else  {
            Ok(TicketDraft {
                title: title,
                description: description
            })
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

}



#[cfg(test)]
mod tests {
    use crate::models::{TicketDraft};
    use fake::Fake;

    #[test]
    fn creating_a_ticket_with_title_should_not_fail() {
        
        //arrange
        let faker = fake::Faker;

        let title: String =  faker.fake();
        let description: String = faker.fake();

        let expected_title = title.to_owned();
        let expected_description = description.to_owned();

        //act
        let new_ticket_draft = TicketDraft::new(title,description).expect("Ticket Draft should exist");

        assert_eq!(new_ticket_draft.get_title().to_string(), expected_title);
        assert_eq!(new_ticket_draft.get_description().to_string(), expected_description);

    }

    #[test]
    #[should_panic]
    fn creating_a_ticket_with_no_title_should_fail() {
        
        //arrange
        let faker = fake::Faker;

        let title: String =  faker.fake();
        let description: String = faker.fake();

        let expected_title = title.to_owned();
        let expected_description = description.to_owned();

        //act
        let new_ticket_draft = TicketDraft::new("".to_string(), "".to_string()).expect("Ticket Draft should exist");

        assert_eq!(new_ticket_draft.get_title().to_string(), expected_title);
        assert_eq!(new_ticket_draft.get_description().to_string(), expected_description);

    }

}
