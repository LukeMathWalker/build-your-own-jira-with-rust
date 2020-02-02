use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
/// The title of a [Ticket](Ticket)
/// Wraps a string and checks that it's not empty when set
pub struct Title {
   title : String
}

#[derive(PartialEq, Debug, Clone)]
/// Error if a title cannot be created
pub struct TitleError {
    details: String
}

/// Sets the error message for a title if it cannot be created
impl TitleError {
    fn new(msg: &str) -> TitleError {
        TitleError{details: msg.to_string()}
    }
}

impl fmt::Display for TitleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for TitleError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl Title {
    /// Creates a Title for a [Ticket](Ticket)
    /// results in a [TitleError](TitleError) if the string passed in is empty
    pub fn new(title: String) -> Result<Title, TitleError> {
        
        if title.is_empty() {
            Err(TitleError::new("Title Cannot be empty"))
        } else  {
            Ok(Title {
                title: title,
            })
        }
    }

    /// returns a title for [Ticket](Ticket)
    pub fn get_title(&self) -> &String {
        &self.title
    }

}

#[cfg(test)]
mod title_tests {
    use crate::models::{Title};
    use fake::Fake;

    #[test]
    fn creating_a_ticket_with_title_should_not_fail() {
        
        //arrange
        let faker = fake::Faker;

        let title: String =  faker.fake();

        let expected_title = title.to_owned();

        //act
        let new_title = Title::new(title).expect("Title should exist");

        assert_eq!(new_title.get_title().to_string(), expected_title);
    }

    #[test]
    #[should_panic]
    fn creating_an_empty_title_should_fail() {
        
        //arrange
        let faker = fake::Faker;

        let title: String =  faker.fake();

        let expected_title = title.to_owned();

        //act
        let new_title = Title::new("".to_string()).expect("Title should exist");

        assert_eq!(new_title.get_title().to_string(), expected_title);
    }

}